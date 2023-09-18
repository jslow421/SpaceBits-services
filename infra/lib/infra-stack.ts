import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as functions from "./functions/functions";
import * as iam from "./iam/iam";
import * as api from "./api/api";

export class SpaceRustStack extends cdk.Stack {
	constructor(scope: Construct, id: string, props?: cdk.StackProps) {
		super(scope, id, props);

		const BUCKET_NAME = "spaceclouddatabucket";
		const spaceBitsLambdaRole = iam.createLambdaRole(this);

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
		const spaceBitsApi = api.createRestApi(this);

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
		const spaceBitsApiUsagePlan = api.createApiUsagePlan(this, spaceBitsApi);

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
