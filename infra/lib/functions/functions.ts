import * as cdk from "aws-cdk-lib";
import * as lambda from "aws-cdk-lib/aws-lambda";
import { Role } from "aws-cdk-lib/aws-iam";

class CreateRustLambdaFunctionArm64Props {
	id: string;
	stack: cdk.Stack;
	role: Role;
	bucketName?: string;
	functionName: string;
	fileName: string;
	description?: string;
	environment?: {};
}

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
	return createRustLambdaFunctionArm64({
		id: "GetAndStorePeopleInSpaceFunction",
		stack: stack,
		role: role,
		bucketName: bucketName,
		functionName: "get-and-store-people-in-space",
		fileName: "getpeopleinspacedata",
		description: "This function gets the people in space data and stores it in the bucket.",
		environment: {
			BUCKET_NAME: bucketName,
			FILE_NAME: "people_in_space.json",
		},
	});
}

// Retrieve Near Earth Objects from stored JSON
export function retrieveNearEarthObjectsFunction(stack: cdk.Stack, role: Role, bucketName: string) {
	return createRustLambdaFunctionArm64({
		id: "RetrieveNearEarthObjectsFunction",
		stack: stack,
		role: role,
		bucketName: bucketName,
		functionName: "RetrieveStoredNEOJsonForApi",
		fileName: "retrievenearearthobjects",
		environment: {
			BUCKET_NAME: bucketName,
		},
	});
}

export function nearEarthObjectsRetrievalFunction(stack: cdk.Stack, role: Role, bucketName: string) {
	return createRustLambdaFunctionArm64({
		id: "NearEarthObjectsRustFunction",
		stack: stack,
		role: role,
		bucketName: bucketName,
		functionName: "retrieve-near-earth-objects-rust",
		fileName: "readnearearthobjects",
		environment: {
			BUCKET_NAME: bucketName,
			FILE_NAME: "near_earth_objects.json",
			KEY_LOCATION: "/space_cloud/keys/nasa_api_key",
		},
	});
}

function createRustLambdaFunctionArm64(props: CreateRustLambdaFunctionArm64Props) {
	return new cdk.aws_lambda.Function(props.stack, props.id, {
		functionName: `${props.stack.stackName}-${props.functionName}`,
		runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
		memorySize: 128,
		timeout: cdk.Duration.seconds(30),
		code: cdk.aws_lambda.Code.fromAsset("../functions/out/" + props.fileName),
		handler: "nil",
		architecture: cdk.aws_lambda.Architecture.ARM_64,
		role: props.role,
		logRetention: cdk.aws_logs.RetentionDays.ONE_DAY,
		environment: props.environment,
		description: props.description,
	});
}
