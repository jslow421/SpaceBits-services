use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use log::LevelFilter;
use reqwest;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
mod models;

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    links: ApiResponseLink,
    element_count: i32,
    near_earth_objects: Vec<NearEarthObjects>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NearEarthObjects {
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
struct ApiResponseLink {
    next: String,
    prev: String,
    this: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NearEarthObject {
    id: String,
    name: String,
    nasa_jpl_url: String,
    is_potentially_hazardous_asteroid: bool,
    absolute_magnitude_h: f64,
    estimated_diameter: EstimatedDiameter,
    close_approach_data: Vec<CloseApproachData>,
    is_sentry_object: bool,
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

async fn retrieveData() -> Result<ApiResponse, Error> {
    let client = reqwest::Client::new();
    // let res: reqwest::Response = client
    //     .get("https://api.nasa.gov/neo/rest/v1/feed?")
    //     .send()
    //     .await
    //     .map_err(Box::new)?;

    let json_response = client
        .get("https://api.nasa.gov/neo/rest/v1/feed?")
        .send()
        .await?
        .text()
        .await?;

    let data = serde_json::from_str(&json_response)?;

    Ok(data)
}

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let data = retrieveData().await?;
    let serialized_data = serde_json::to_string(&data).unwrap();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(serialized_data.into())
        .map_err(Box::new)?;
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
