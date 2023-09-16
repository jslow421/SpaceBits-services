import * as cdk from "aws-cdk-lib";
import * as lambda from "aws-cdk-lib/aws-lambda";
import { Role } from "aws-cdk-lib/aws-iam";

export function getUpcomingLaunchJsonForApi(stack: cdk.Stack, role: Role, bucketName: string): lambda.Function {
	return new cdk.aws_lambda.Function(stack, "getUpcomingLaunchJsonForApi", {
		functionName: `${stack.stackName}-get-upcoming-launch-json-for-api`,
		runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
		memorySize: 128,
		timeout: cdk.Duration.seconds(30),
		code: cdk.aws_lambda.Code.fromAsset("../functions/out/readupcominglaunches"),
		handler: "nil",
		architecture: cdk.aws_lambda.Architecture.ARM_64,
		role: role,
		logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
		environment: {
			BUCKET_NAME: bucketName,
			FILE_NAME: "launches.json",
		},
	});
}

export function readFunction(stack: cdk.Stack, role: Role) {
	return new cdk.aws_lambda.Function(stack, "ReadPeopleInSpaceRustFunction", {
		functionName: "read-people-in-space-rust",
		runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
		memorySize: 128,
		timeout: cdk.Duration.seconds(30),
		code: cdk.aws_lambda.Code.fromAsset("../functions/out/readpeople"),
		handler: "nil",
		architecture: cdk.aws_lambda.Architecture.ARM_64,
		role: role,
		logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
	});
}

export function getAndStorePeopleInSpaceFunction(stack: cdk.Stack, role: Role, bucketName: string) {
	return new cdk.aws_lambda.Function(stack, "GetAndStorePeopleInSpaceFunction", {
		functionName: "get-and-store-people-in-space",
		runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
		memorySize: 128,
		timeout: cdk.Duration.seconds(30),
		code: cdk.aws_lambda.Code.fromAsset("../functions/out/getpeopleinspacedata"),
		handler: "nil",
		architecture: cdk.aws_lambda.Architecture.ARM_64,
		role: role,
		logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
		environment: {
			BUCKET_NAME: bucketName,
			FILE_NAME: "people_in_space.json",
		},
	});
}

// Retrieve Near Earth Objects from stored JSON
export function retrieveNearEarthObjectsFunction(stack: cdk.Stack, role: Role, bucketName: string) {
	return new cdk.aws_lambda.Function(stack, "RetrieveNearEarthObjectsFunction", {
		functionName: `${stack.stackName}-RetrieveStoredNEOJsonForApi`,
		runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
		memorySize: 128,
		timeout: cdk.Duration.seconds(30),
		code: cdk.aws_lambda.Code.fromAsset("../functions/out/retrievenearearthobjects"),
		handler: "nil",
		architecture: cdk.aws_lambda.Architecture.ARM_64,
		role: role,
		logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
		environment: {
			BUCKET_NAME: bucketName,
		},
	});
}

export function nearEarthObjectsRetrievalFunction(stack: cdk.Stack, role: Role, bucketName: string) {
	return new cdk.aws_lambda.Function(stack, "NearEarthObjectsRustFunction", {
		functionName: `${stack.stackName}-retrieve-near-earth-objects-rust`,
		runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
		memorySize: 128,
		timeout: cdk.Duration.seconds(30),
		code: cdk.aws_lambda.Code.fromAsset("../functions/out/readnearearthobjects"),
		handler: "nil",
		architecture: cdk.aws_lambda.Architecture.ARM_64,
		role: role,
		logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
		environment: {
			BUCKET_NAME: bucketName,
			FILE_NAME: "near_earth_objects.json",
			KEY_LOCATION: "/space_cloud/keys/nasa_api_key",
		},
	});
}
