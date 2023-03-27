use aws_config;
use aws_sdk_s3 as s3;
use aws_sdk_ssm::{Client as ssm_client, Error as ssm_error, Region, PKG_VERSION};
use chrono::{DateTime, Utc};
use s3::types::ByteStream;
use std::collections::HashMap;

use lambda_http::{run, service_fn, Body, Error, Request, Response};
use log::LevelFilter;
use reqwest;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    links: ApiResponseLink,
    element_count: i32,
    near_earth_objects: HashMap<String, Vec<NearEarthObject>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NearEarthObject {
    id: String,
    neo_reference_id: String,
    name: String,
    nasa_jpl_url: String,
    absolute_magnitude_h: f64,
    estimated_diameter: EstimatedDiameter,
    is_potentially_hazardous_asteroid: bool,
    close_approach_data: Vec<CloseApproachData>,
    is_sentry_object: bool,
    links: ApiResponseLink,
}

#[derive(Debug, Serialize, Deserialize)]
struct NearEarthLink {
    #[serde(alias = "self")]
    this: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponseLink {
    next: Option<String>,
    prev: Option<String>,
    #[serde(alias = "self")]
    this: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EstimatedDiameter {
    kilometers: EstimatedDiameterValues,
    meters: EstimatedDiameterValues,
    miles: EstimatedDiameterValues,
    feet: EstimatedDiameterValues,
}

#[derive(Debug, Serialize, Deserialize)]
struct EstimatedDiameterValues {
    estimated_diameter_min: f64,
    estimated_diameter_max: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloseApproachData {
    close_approach_date: String,
    epoch_date_close_approach: i64,
    relative_velocity: RelativeVelocity,
    miss_distance: MissDistance,
    orbiting_body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RelativeVelocity {
    kilometers_per_second: String,
    kilometers_per_hour: String,
    miles_per_hour: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MissDistance {
    astronomical: String,
    lunar: String,
    kilometers: String,
    miles: String,
}

async fn retrieve_data() -> Result<ApiResponse, Error> {
    let client = reqwest::Client::new();
    let secrets_client = ssm_client::new(&aws_config::load_from_env().await);
    let key_val = secrets_client
        .get_parameter()
        .name("/space_cloud/keys/nasa_api_key")
        .with_decryption(true)
        .send()
        .await?
        .parameter
        .unwrap()
        .value
        .unwrap();

    let today = Utc::now().format("%Y-%m-%d");
    let start_date = String::from(today.to_string());
    let end_date = String::from(today.to_string());

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

    println!("{:?}", json_response);

    let data = serde_json::from_str(&json_response)?;

    return Ok(data);
}

async fn write_to_s3(data: ApiResponse) {
    let config = aws_config::load_from_env().await;
    let client = s3::Client::new(&config);

    let json = serde_json::to_string(&data).unwrap();
    let stream = ByteStream::from(json.as_bytes().to_vec());

    let _data = client
        .put_object()
        .bucket("spaceclouddatabucket")
        .key("near_earth_objects.json")
        .body(stream)
        .send()
        .await;
}

async fn function_handler(_event: Request) -> Result<Response<String>, Error> {
    let data = retrieve_data().await?;
    //let serialized_data = serde_json::to_string(&data).unwrap();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(String::from("Completed"))
        .map_err(Box::new)?;

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
        assert!(data.near_earth_objects.len() >= 1);
    }
}
