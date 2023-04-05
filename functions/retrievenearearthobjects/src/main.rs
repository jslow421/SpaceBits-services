use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use shared::{ApiResponse, StoredNearEarthObjectModel};

async fn get_json_data_from_s3() -> Result<ApiResponse, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    let data = client
        .get_object()
        .bucket("spaceclouddatabucket")
        .key("near_earth_objects.json")
        .send()
        .await?;

    let bytes = data.body.collect().await?.into_bytes();
    let response = std::str::from_utf8(&bytes)?;
    let temp: ApiResponse = serde_json::from_str(response).unwrap();

    Ok(temp)
}

fn generate_response(data: ApiResponse) -> StoredNearEarthObjectModel {
    StoredNearEarthObjectModel {
        updated_date_time: "COMING_SOON".to_string(),
        data,
    }
}

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let data = get_json_data_from_s3().await.unwrap();
    let response = generate_response(data);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "GET")
        .body(serde_json::to_string(&response).unwrap_or_default().into())
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
