use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use shared::persistencemodels::UpcomingLaunches;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let launches = retrieve_json_data_from_s3().await?;

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&launches).unwrap_or_default().into())
        .map_err(Box::new)?;
    Ok(resp)
}

async fn generate

async fn retrieve_json_data_from_s3() -> Result<UpcomingLaunches, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    let data = client
        .get_object()
        .bucket("my-bucket")
        .key("my-key")
        .send()
        .await?;

    let bytes = data.body.collect().await?.into_bytes();
    let response = std::str::from_utf8(&bytes)?;
    let launches: UpcomingLaunches = serde_json::from_str(response).unwrap();

    Ok(launches)
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
