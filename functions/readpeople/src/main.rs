// use aws_sdk_s3::{Client, Region};
use aws_config;
use aws_sdk_s3 as s3;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Person {
    name: String,
    craft: Craft,
}

#[derive(Deserialize, Serialize, Debug)]
struct PeopleInSpaceResponse {
    #[serde(alias = "updatedTime")]
    update_date: String,
    people: Vec<Person>,
}

#[derive(Deserialize, Serialize, Debug)]
enum Craft {
    #[serde(alias = "ISS")]
    ISS,
    #[serde(alias = "Shenzhou 15")]
    Shenzhou15,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let client = s3::Client::new(&config);
    let data = client
        .get_object()
        .bucket("spaceclouddatabucket")
        .key("people_in_space.json")
        .send()
        .await?;

    ////log::info!("{}", data.content_length());

    let bytes = data.body.collect().await?.into_bytes();
    let response = std::str::from_utf8(&bytes)?;
    let temp: PeopleInSpaceResponse = serde_json::from_str(response).unwrap();

    log::info!("Updated date for found values: {}", temp.update_date);

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header(
            "Access-Control-Allow-Headers",
            "Content-Type,Authorization,",
        )
        .header("Access-Control-Allow-Methods", "GET")
        .body(serde_json::to_string(&temp).unwrap().into())
        .map_err(Box::new)?;

    return Ok(resp);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
