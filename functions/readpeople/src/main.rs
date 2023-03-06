// use aws_sdk_s3::{Client, Region};
use aws_config;
use aws_sdk_s3 as s3;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use structopt::StructOpt;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Person {
    name: String,
    craft: Craft,
}

#[derive(Deserialize, Debug)]
struct PeopleInSpaceResponse {
    #[serde(alias = "updateDate")]
    update_date: String,
    people: Vec<Person>,
}

#[derive(Deserialize, Serialize, Debug)]
enum Craft {
    ISS,
    Hubble,
    Other(String),
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the bucket.
    #[structopt(short, long)]
    bucket: String,

    /// The object key.
    #[structopt(short, long)]
    object: String,

    /// How long in seconds before the presigned request should expire.
    #[structopt(short, long)]
    expires_in: Option<u64>,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    // Extract some useful information from the request

    let thing = PeopleInSpaceResponse {
        update_date: String::from("2021-01-01"),
        people: vec![Person {
            name: String::from("John"),
            craft: Craft::ISS,
        }],
    };

    let config = aws_config::load_from_env().await;
    let client = s3::Client::new(&config);

    let data = client
        .get_object()
        .bucket("spaceclouddatabucket")
        .key("people_in_space.json")
        .send()
        .await?;

    log::info!("{}", data.content_length());

    log::info!("{}", thing.people[0].name);

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request".into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
