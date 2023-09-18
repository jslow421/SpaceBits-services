import * as cdk from "aws-cdk-lib";

export function createLambdaRole(stack: cdk.Stack) {
	return new cdk.aws_iam.Role(stack, "SpaceBitsLambdaRole", {
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
}
