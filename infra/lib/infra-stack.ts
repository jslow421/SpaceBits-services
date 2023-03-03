import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const existingRole = cdk.aws_iam.Role.fromRoleArn(
      this,
      "ExistingRole",
      "arn:aws:iam::939984321277:role/SpaceCloudStack-S3LambdaRoleB1E5D5C9-17FX54CGH1K7F"
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
        code: cdk.aws_lambda.Code.fromAsset("../functions/out/"),
        handler: "read-people-in-space-rust",
        architecture: cdk.aws_lambda.Architecture.ARM_64,
        role: existingRole,
        logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
      }
    );

    // Get existing bucket
    const bucket = cdk.aws_s3.Bucket.fromBucketName(
      this,
      "SpaceCloudBucket",
      "spaceclouddatabucket"
    );

    bucket.grantRead(readFunction);
  }
}
