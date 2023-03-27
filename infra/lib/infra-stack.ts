import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export class SpaceRustStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const spacecloudLambdaRole = cdk.aws_iam.Role.fromRoleArn(
      this,
      "ExistingRole",
      "arn:aws:iam::939984321277:role/SpaceCloudStack-S3LambdaRoleB1E5D5C9-17FX54CGH1K7F"
    );

    spacecloudLambdaRole.attachInlinePolicy(
      new cdk.aws_iam.Policy(this, "SpaceCloudS3Policy", {
        statements: [
          new cdk.aws_iam.PolicyStatement({
            effect: cdk.aws_iam.Effect.ALLOW,
            actions: ["ssm:GetParameter", "ssm:Decrypt"],
            resources: ["*"],
          }),
        ],
      })
    );

    // Functions
    // Read function in Rust
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
        role: spacecloudLambdaRole,
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
        role: spacecloudLambdaRole,
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

    // Grant permissions for bucket to functions
    bucket.grantRead(readFunction);
    bucket.grantReadWrite(nearEarthObjectsRetrievalFunction);

    // Event to run daily
    const dailyEventRule = new cdk.aws_events.Rule(this, "dailyEventRule", {
      schedule: cdk.aws_events.Schedule.cron({
        minute: "0",
        hour: "5",
      }),
      enabled: true,
    });

    dailyEventRule.addTarget(
      new cdk.aws_events_targets.LambdaFunction(
        nearEarthObjectsRetrievalFunction
      )
    );
  }
}
