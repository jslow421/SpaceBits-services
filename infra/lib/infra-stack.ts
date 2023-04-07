import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export class SpaceRustStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const spaceBitsLambdaRole = new cdk.aws_iam.Role(
      this,
      "SpaceBitsLambdaRole",
      {
        assumedBy: new cdk.aws_iam.ServicePrincipal("lambda.amazonaws.com"),
        managedPolicies: [
          cdk.aws_iam.ManagedPolicy.fromAwsManagedPolicyName(
            "AmazonDynamoDBFullAccess"
          ),
          cdk.aws_iam.ManagedPolicy.fromAwsManagedPolicyName(
            "CloudWatchFullAccess"
          ),
        ],
        inlinePolicies: {
          SpaceBitsLambdaPolicy: new cdk.aws_iam.PolicyDocument({
            statements: [
              new cdk.aws_iam.PolicyStatement({
                effect: cdk.aws_iam.Effect.ALLOW,
                actions: ["ssm:GetParameter", "ssm:Decrypt"],
                resources: ["*"],
              }),
            ],
          }),
        },
      }
    );

    // Functions
    // Read function
    const readFunction = new cdk.aws_lambda.Function(
      this,
      "ReadPeopleInSpaceRustFunction",
      {
        functionName: "read-people-in-space-rust",
        runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
        memorySize: 128,
        timeout: cdk.Duration.seconds(30),
        code: cdk.aws_lambda.Code.fromAsset("../functions/out/readpeople"),
        handler: "nil",
        architecture: cdk.aws_lambda.Architecture.ARM_64,
        role: spaceBitsLambdaRole,
        logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
      }
    );

    // Get and store NEO for the day
    const nearEarthObjectsRetrievalFunction = new cdk.aws_lambda.Function(
      this,
      "NearEarthObjectsRustFunction",
      {
        functionName: "retrieve-near-earth-objects-rust",
        runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
        memorySize: 128,
        timeout: cdk.Duration.seconds(30),
        code: cdk.aws_lambda.Code.fromAsset(
          "../functions/out/readnearearthobjects"
        ),
        handler: "nil",
        architecture: cdk.aws_lambda.Architecture.ARM_64,
        role: spaceBitsLambdaRole,
        logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
        environment: {
          BUCKET_NAME: "spaceclouddatabucket",
        },
      }
    );

    // Retrieve Near Earth Objects from stored JSON
    const retrieveNearEarthObjectsFunction = new cdk.aws_lambda.Function(
      this,
      "RetrieveNearEarthObjectsFunction",
      {
        functionName: "retrieve-near-earth-objects",
        runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
        memorySize: 128,
        timeout: cdk.Duration.seconds(30),
        code: cdk.aws_lambda.Code.fromAsset(
          "../functions/out/retrievenearearthobjects"
        ),
        handler: "nil",
        architecture: cdk.aws_lambda.Architecture.ARM_64,
        role: spaceBitsLambdaRole,
        logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
        environment: {
          BUCKET_NAME: "spaceclouddatabucket",
        },
      }
    );

    // Get and store people in space data
    const getAndStorePeopleInSpaceFunction = new cdk.aws_lambda.Function(
      this,
      "GetAndStorePeopleInSpaceFunction",
      {
        functionName: "get-and-store-people-in-space",
        runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
        memorySize: 128,
        timeout: cdk.Duration.seconds(30),
        code: cdk.aws_lambda.Code.fromAsset(
          "../functions/out/getpeopleinspacedata"
        ),
        handler: "nil",
        architecture: cdk.aws_lambda.Architecture.ARM_64,
        role: spaceBitsLambdaRole,
        logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
        environment: {
          BUCKET_NAME: "spaceclouddatabucket",
        },
      }
    );

    // Get existing bucket
    const bucket = cdk.aws_s3.Bucket.fromBucketName(
      this,
      "SpaceCloudBucket",
      "spaceclouddatabucket"
    );

    // Get existing API Gateway
    const api = cdk.aws_apigateway.RestApi.fromRestApiId(
      this,
      "SpaceCloudAPI",
      "yqc46mujr3"
    );

    // Grant permissions for bucket to functions
    bucket.grantRead(readFunction);
    bucket.grantRead(retrieveNearEarthObjectsFunction);
    bucket.grantReadWrite(nearEarthObjectsRetrievalFunction);

    // Api Gateway
    const spaceBitsApi = new cdk.aws_apigateway.RestApi(this, "SpaceBitsApi", {
      restApiName: "SpaceBits API",
      description: "This API serves the SpaceBits app.",
      retainDeployments: false,
      endpointExportName: "SpaceBitsApiEndpoint",
      deploy: true,
      endpointConfiguration: {
        types: [cdk.aws_apigateway.EndpointType.REGIONAL],
      },
      deployOptions: {
        stageName: "prod",
        cacheDataEncrypted: false,
        throttlingBurstLimit: 100,
        throttlingRateLimit: 1000,
        loggingLevel: cdk.aws_apigateway.MethodLoggingLevel.INFO,
      },
      domainName: {
        domainName: "api.spacebits.net",
        certificate: cdk.aws_certificatemanager.Certificate.fromCertificateArn(
          this,
          "SpaceBitsCertificate",
          "arn:aws:acm:us-east-1:939984321277:certificate/32c01289-82c7-4d30-885a-d5cd3aab4a93"
        ),
      },
      defaultCorsPreflightOptions: {
        allowOrigins: cdk.aws_apigateway.Cors.ALL_ORIGINS,
        allowMethods: cdk.aws_apigateway.Cors.ALL_METHODS,
        allowHeaders: cdk.aws_apigateway.Cors.DEFAULT_HEADERS,
      },
    });

    // Read people endpoint
    const readPeopleResource = spaceBitsApi.root.addResource("people");
    readPeopleResource.addMethod(
      "GET",
      new cdk.aws_apigateway.LambdaIntegration(readFunction),
      {
        apiKeyRequired: true,
      }
    );

    // Retrieve Near Earth Objects endpoint
    const retrieveNearEarthObjectsResource =
      spaceBitsApi.root.addResource("neo");
    retrieveNearEarthObjectsResource.addMethod(
      "GET",
      new cdk.aws_apigateway.LambdaIntegration(
        retrieveNearEarthObjectsFunction
      ),
      {
        apiKeyRequired: true,
      }
    );

    // API usage plan
    const spaceBitsApiUsagePlan = new cdk.aws_apigateway.UsagePlan(
      this,
      "SpaceBitsApiUsagePlan",
      {
        name: "SpaceBitsApiUsagePlan",
        throttle: {
          rateLimit: 1000,
          burstLimit: 10,
        },
        quota: {
          limit: 10000,
          offset: 0,
          period: cdk.aws_apigateway.Period.DAY,
        },
        apiStages: [
          {
            api: spaceBitsApi,
            stage: spaceBitsApi.deploymentStage,
            throttle: [],
          },
        ],
      }
    );

    // Event rules
    const everyTwoHoursEventRule = new cdk.aws_events.Rule(
      this,
      "everyTwoHoursEventRule",
      {
        schedule: cdk.aws_events.Schedule.rate(cdk.Duration.hours(2)),
        enabled: true,
        ruleName: "everyTwoHoursEventRule",
      }
    );

    const twiceDailyEventRule = new cdk.aws_events.Rule(
      this,
      "twiceDailyEventRule",
      {
        schedule: cdk.aws_events.Schedule.cron({
          hour: "0/12",
          minute: "0",
        }),
      }
    );

    // Event to run daily
    const dailyEventRule = new cdk.aws_events.Rule(this, "dailyEventRule", {
      schedule: cdk.aws_events.Schedule.cron({
        minute: "0",
        hour: "5",
      }),
      enabled: true,
    });

    // Add targets to event rules
    everyTwoHoursEventRule.addTarget(
      new cdk.aws_events_targets.LambdaFunction(
        getAndStorePeopleInSpaceFunction
      )
    );

    dailyEventRule.addTarget(
      new cdk.aws_events_targets.LambdaFunction(
        nearEarthObjectsRetrievalFunction
      )
    );

    // Key for API usage plan
    const apiKey = spaceBitsApi.addApiKey("SpaceBitsApiKey");
    spaceBitsApiUsagePlan.addApiKey(apiKey);
  }
}
