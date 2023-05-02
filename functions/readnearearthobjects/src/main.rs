use aws_sdk_s3 as s3;
use aws_sdk_ssm::Client as ssm_client;
use chrono::Utc;
use lambda_http::{run, service_fn, Error, Request, Response};
use log::LevelFilter;
use s3::primitives::ByteStream;
use shared::apimodels::ApiResponse;
use shared::persistencemodels::*;
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

async fn convert_to_storage(response: ApiResponse) -> Result<NearEarthObjectModel, Error> {
    let mut near_earth_objects: NearEarthObjectModel = NearEarthObjectModel {
        links: ApiLinks {
            next: response.links.next,
            prev: response.links.prev,
            this: response.links.this,
        },
        element_count: response.element_count,
        near_earth_objects: Vec::new(),
    };

    let first_item = response.near_earth_objects.values().next().unwrap();

    // TODO: Since we're going through this effort, maybe we can clean up the types so we're not cloning all these strings
    for item in first_item {
        let mut neo = NearEarthObject {
            id: item.id.clone(),
            neo_reference_id: item.neo_reference_id.clone(),
            name: item.name.clone(),
            nasa_jpl_url: item.nasa_jpl_url.clone(),
            absolute_magnitude_h: item.absolute_magnitude_h,
            estimated_diameter: EstimatedDiameter {
                kilometers: EstimatedDiameterValues {
                    estimated_diameter_min: item
                        .estimated_diameter
                        .kilometers
                        .estimated_diameter_min,
                    estimated_diameter_max: item
                        .estimated_diameter
                        .kilometers
                        .estimated_diameter_max,
                },
                meters: EstimatedDiameterValues {
                    estimated_diameter_min: item.estimated_diameter.meters.estimated_diameter_min,
                    estimated_diameter_max: item.estimated_diameter.meters.estimated_diameter_max,
                },
                miles: EstimatedDiameterValues {
                    estimated_diameter_min: item.estimated_diameter.miles.estimated_diameter_min,
                    estimated_diameter_max: item.estimated_diameter.miles.estimated_diameter_max,
                },
                feet: EstimatedDiameterValues {
                    estimated_diameter_min: item.estimated_diameter.feet.estimated_diameter_min,
                    estimated_diameter_max: item.estimated_diameter.feet.estimated_diameter_max,
                },
            },
            is_potentially_hazardous_asteroid: item.is_potentially_hazardous_asteroid,
            close_approach_data: Vec::new(),
            is_sentry_object: item.is_sentry_object,
            links: ApiLinks {
                next: item.links.next.clone(),
                prev: item.links.prev.clone(),
                this: item.links.this.clone(),
            },
        };

        for close_approach in &item.close_approach_data {
            let close_approach_data = CloseApproachData {
                close_approach_date: close_approach.close_approach_date.clone(),
                orbiting_body: close_approach.orbiting_body.clone(),
                epoch_date_close_approach: close_approach.epoch_date_close_approach,
                relative_velocity: RelativeVelocity {
                    kilometers_per_second: close_approach
                        .relative_velocity
                        .kilometers_per_second
                        .clone(),
                    kilometers_per_hour: close_approach
                        .relative_velocity
                        .kilometers_per_hour
                        .clone(),
                    miles_per_hour: close_approach.relative_velocity.miles_per_hour.clone(),
                },
                miss_distance: MissDistance {
                    astronomical: close_approach.miss_distance.astronomical.clone(),
                    lunar: close_approach.miss_distance.lunar.clone(),
                    kilometers: close_approach.miss_distance.kilometers.clone(),
                    miles: close_approach.miss_distance.miles.clone(),
                },
            };

            neo.close_approach_data.push(close_approach_data);
        }

        near_earth_objects.near_earth_objects.push(neo);
    }

    Ok(near_earth_objects)
}

/// Persist the data to S3
async fn write_to_s3(data: NearEarthObjectModel) {
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
    let converted_data = convert_to_storage(data).await?;
    write_to_s3(converted_data).await;

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
