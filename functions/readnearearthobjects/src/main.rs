use aws_sdk_s3 as s3;
use aws_sdk_ssm::Client as ssm_client;
use chrono::Utc;
use lambda_http::{run, service_fn, Error, Request, Response};
use log::LevelFilter;
use s3::primitives::ByteStream;
use shared::ApiResponse;
use simple_logger::SimpleLogger;
use std::env;

async fn retrieve_data() -> Result<ApiResponse, Error> {
    let client = reqwest::Client::new();
    let secrets_client = ssm_client::new(&aws_config::load_from_env().await);
    let key_location = env::var("KEY_LOCATION").unwrap();
    let key_val = secrets_client
        .get_parameter()
        .name(key_location)
        .with_decryption(true)
        .send()
        .await?
        .parameter
        .unwrap()
        .value
        .unwrap();

    let today = Utc::now().format("%Y-%m-%d");
    let start_date = today.to_string();
    let end_date = today.to_string();

    let response = client
        .get(format!(
            "https://api.nasa.gov/neo/rest/v1/feed?start_date={}&end_date={}&api_key={}",
            start_date, end_date, key_val
        ))
        .send()
        .await?;

    if response.status() != 200 {
        return Err(Error::from(format!(
            "Error retrieving data from NASA API: {}",
            response.status()
        )));
    }

    let json_response = response.text().await?;

    let data = serde_json::from_str(&json_response)?;

    Ok(data)
}

/// Persist the data to S3
async fn write_to_s3(data: ApiResponse) {
    let config = aws_config::load_from_env().await;
    let client = s3::Client::new(&config);

    let json = serde_json::to_string(&data).unwrap();
    let stream = ByteStream::from(json.as_bytes().to_vec());

    let bucket_name = env::var("BUCKET_NAME").unwrap();
    let file_name = env::var("FILE_NAME").unwrap();

    let _data = client
        .put_object()
        .bucket(bucket_name)
        .key(file_name)
        .body(stream)
        .send()
        .await;
}

async fn function_handler(_event: Request) -> Result<Response<String>, Error> {
    let data = retrieve_data().await?;
    let resp = Response::new("".to_string());

    write_to_s3(data).await;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retrieve_dat_element_count_greater_than_zero() {
        let data = retrieve_data().await.unwrap();
        println!("{:?}", data);
        assert!(data.element_count > 0);
    }

    #[tokio::test]
    async fn test_retrieve_data_objects_greater_equal_one() {
        let data = retrieve_data().await.unwrap();
        println!("{:?}", data);
        assert!(!data.near_earth_objects.is_empty());
    }
}
