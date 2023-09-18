import * as cdk from "aws-cdk-lib";
import { RestApiProps, UsagePlanProps } from "aws-cdk-lib/aws-apigateway";

export function createRestApi(stack: cdk.Stack): cdk.aws_apigateway.RestApi {
	return new cdk.aws_apigateway.RestApi(stack, "SpaceBitsApi", {
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
				stack,
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
}

export function createApiUsagePlan(
	stack: cdk.Stack,
	spaceBitsApi: cdk.aws_apigateway.RestApi,
): cdk.aws_apigateway.UsagePlan {
	return new cdk.aws_apigateway.UsagePlan(stack, "SpaceBitsApiUsagePlan", {
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
}
