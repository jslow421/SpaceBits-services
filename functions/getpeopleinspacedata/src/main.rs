use aws_sdk_s3 as s3;
use chrono::prelude::*;
use lambda_http::{run, service_fn, Error, Request, Response};
use log::LevelFilter;
use s3::primitives::ByteStream;
use serde::{Deserialize, Serialize};
use shared::{PeopleInSpaceModel, PersonModel};
use simple_logger::SimpleLogger;
use std::env;

#[derive(Deserialize, Serialize)]
struct PeopleApiResponse {
    message: String,
    people: Vec<PersonModel>,
}

async fn retrieve_data_from_api() -> Result<PeopleApiResponse, Error> {
    log::info!("Retrieving data from API");
    let response = reqwest::get("http://api.open-notify.org/astros.json").await?;
    let json_body = response.json::<serde_json::Value>().await?;
    let model: PeopleApiResponse = serde_json::from_value(json_body).unwrap();
    log::info!("Successfully retrieved data from API");
    Ok(model)
}

async fn write_to_s3(data: PeopleInSpaceModel) -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    let bucket_name = env::var("BUCKET_NAME").unwrap();
    let file_name = env::var("FILE_NAME").unwrap();
    let json = serde_json::to_string(&data).unwrap_or_else(|err| {
        panic!("Error serializing data to JSON: {}", err);
    });

    let bytes = ByteStream::from(json.as_bytes().to_vec());

    client
        .put_object()
        .bucket(bucket_name)
        .key(file_name)
        .body(bytes)
        .send()
        .await?;

    Ok(())
}

fn convert_to_model(api_response: PeopleApiResponse) -> PeopleInSpaceModel {
    PeopleInSpaceModel {
        update_time: Utc::now().format("%Y-%m-%d %H:%M:%S %z").to_string(),
        people: api_response.people,
    }
}

async fn function_handler(_event: Request) -> Result<Response<String>, Error> {
    log::info!("Starting function handler");
    let api_response = retrieve_data_from_api().await?;
    log::info!("Retrieved data from API");
    let data = convert_to_model(api_response);
    log::info!("Writing data to S3");
    write_to_s3(data).await?;

    Ok(Response::new("".to_string()))
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test retrieve_data_from_api
    #[tokio::test]
    async fn api_response_ok() {
        let value = retrieve_data_from_api().await;
        assert!(value.is_ok());
    }

    #[tokio::test]
    async fn api_response_has_value() {
        let value = retrieve_data_from_api().await;
        assert!(!value.unwrap().people.is_empty());
    }

    #[tokio::test]
    async fn can_convert_api_response_to_model() {
        let api_response = retrieve_data_from_api().await;

        match api_response {
            Ok(response) => {
                let model = convert_to_model(response);
                assert!(!model.people.is_empty());
            }
            Err(_) => assert!(false),
        }
    }
}
