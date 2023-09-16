import * as cdk from "aws-cdk-lib";
import { RestApiProps, UsagePlanProps } from "aws-cdk-lib/aws-apigateway";
import { Construct } from "constructs";
import * as functions from "./functions/functions";

export class SpaceRustStack extends cdk.Stack {
	constructor(scope: Construct, id: string, props?: cdk.StackProps) {
		super(scope, id, props);

		const BUCKET_NAME = "spaceclouddatabucket";

		const spaceBitsLambdaRole = new cdk.aws_iam.Role(this, "SpaceBitsLambdaRole", {
			assumedBy: new cdk.aws_iam.ServicePrincipal("lambda.amazonaws.com"),
			managedPolicies: [
				cdk.aws_iam.ManagedPolicy.fromAwsManagedPolicyName("AmazonDynamoDBFullAccess"),
				cdk.aws_iam.ManagedPolicy.fromAwsManagedPolicyName("CloudWatchFullAccess"),
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
		});

		//! Functions
		// Read function
		const readFunction = functions.readFunction(this, spaceBitsLambdaRole);

		// Get and store NEO for the day
		const nearEarthObjectsRetrievalFunction = functions.nearEarthObjectsRetrievalFunction(
			this,
			spaceBitsLambdaRole,
			BUCKET_NAME,
		);

		// Retrieve Near Earth Objects from stored JSON
		const retrieveNearEarthObjectsFunction = functions.retrieveNearEarthObjectsFunction(
			this,
			spaceBitsLambdaRole,
			BUCKET_NAME,
		);

		// Get and store people in space data
		const getAndStorePeopleInSpaceFunction = functions.getAndStorePeopleInSpaceFunction(
			this,
			spaceBitsLambdaRole,
			BUCKET_NAME,
		);

		// Get upcoming launch JSON for API
		const getUpcomingLaunchJsonForApi = functions.getUpcomingLaunchJsonForApi(
			this,
			spaceBitsLambdaRole,
			BUCKET_NAME,
		);

		// Get existing bucket
		const bucket = cdk.aws_s3.Bucket.fromBucketName(this, "SpaceCloudBucket", "spaceclouddatabucket");

		// Grant permissions for bucket to functions
		bucket.grantRead(readFunction);
		bucket.grantRead(retrieveNearEarthObjectsFunction);
		bucket.grantRead(getUpcomingLaunchJsonForApi);
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
					"arn:aws:acm:us-east-1:939984321277:certificate/32c01289-82c7-4d30-885a-d5cd3aab4a93",
				),
			},
			defaultCorsPreflightOptions: {
				allowOrigins: cdk.aws_apigateway.Cors.ALL_ORIGINS,
				allowMethods: cdk.aws_apigateway.Cors.ALL_METHODS,
				allowHeaders: cdk.aws_apigateway.Cors.DEFAULT_HEADERS,
			},
		} as RestApiProps);

		// Read people endpoint
		const readPeopleResource = spaceBitsApi.root.addResource("people");
		readPeopleResource.addMethod("GET", new cdk.aws_apigateway.LambdaIntegration(readFunction), {
			apiKeyRequired: false,
		});

		// Retrieve Near Earth Objects endpoint
		const retrieveNearEarthObjectsResource = spaceBitsApi.root.addResource("neo");
		retrieveNearEarthObjectsResource.addMethod(
			"GET",
			new cdk.aws_apigateway.LambdaIntegration(retrieveNearEarthObjectsFunction),
			{
				apiKeyRequired: true,
			},
		);

		// Get upcoming launches endpoint
		const retrieveUpcomingLaunchesResource = spaceBitsApi.root.addResource("upcomingLaunches");
		retrieveUpcomingLaunchesResource.addMethod(
			"GET",
			new cdk.aws_apigateway.LambdaIntegration(getUpcomingLaunchJsonForApi),
			{
				apiKeyRequired: false,
			},
		);

		// API usage plan
		const spaceBitsApiUsagePlan = new cdk.aws_apigateway.UsagePlan(this, "SpaceBitsApiUsagePlan", {
			name: "SpaceBitsApiUsagePlan",
			description: "Spacebits Api usage plan props",
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
		} as UsagePlanProps);

		// Event rules
		const everyTwoHoursEventRule = new cdk.aws_events.Rule(this, "everyTwoHoursEventRule", {
			schedule: cdk.aws_events.Schedule.rate(cdk.Duration.hours(2)),
			enabled: true,
			ruleName: "everyTwoHoursEventRule",
		});

		// Event to run twice daily
		//    const twiceDailyEventRule = new cdk.aws_events.Rule(
		//      this,
		//      "twiceDailyEventRule",
		//      {
		//        schedule: cdk.aws_events.Schedule.cron({
		//          hour: "0/12",
		//          minute: "0",
		//        }),
		//      }
		//    );

		// Event to run daily
		const dailyEventRule = new cdk.aws_events.Rule(this, "dailyEventRule", {
			schedule: cdk.aws_events.Schedule.cron({
				minute: "0",
				hour: "5",
			}),
			enabled: true,
		});

		// Add targets to event rules
		everyTwoHoursEventRule.addTarget(new cdk.aws_events_targets.LambdaFunction(getAndStorePeopleInSpaceFunction));

		dailyEventRule.addTarget(new cdk.aws_events_targets.LambdaFunction(nearEarthObjectsRetrievalFunction));

		// Key for API usage plan
		const apiKey = spaceBitsApi.addApiKey("SpaceBitsApiKey");
		spaceBitsApiUsagePlan.addApiKey(apiKey);
	}
}
