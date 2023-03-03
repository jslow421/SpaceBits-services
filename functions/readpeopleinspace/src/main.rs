use aws_sdk_s3::{Client, Region, PKG_VERSION};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lambda_runtime;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize)]
// struct Request {
//     pub body: String,
// }
#[derive(Debug, Serialize)]
struct SuccessResponse {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String,
}

// Implement Display for the Failure response so that we can then implement Error.
impl std::fmt::Display for FailureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

// Implement Error for the FailureResponse so that we can `?` (try) the Response
// returned by `lambda_runtime::run(func).await` in `fn main`.
impl std::error::Error for FailureResponse {}

//type Response = Result<SuccessResponse, FailureResponse>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // //let func = service_fn(handler);
    //// lambda_runtime::run(handler).await;

    // Ok(())
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(handler)).await
}

async fn handler(req: Request, _ctx: lambda_runtime::Context, client: &Client) -> Response {
    info!("handling a request...");
    let bucket_name = std::env::var("DATA_BUCKET")
        .expect("A BUCKET_NAME must be set in this app's Lambda environment variables.");
    let file_name = std::env::var("FILE_NAME")
        .expect("A FILE_NAME must be set in this app's Lambda environment variables.");

    // No extra configuration is needed as long as your Lambda has
    // the necessary permissions attached to its role.
    let config = aws_config::load_from_env().await;

    let resp = client
        .get_object()
        .bucket(&bucket_name)
        .key(&file_name)
        .send()
        .await;

    info!(
        "Successfully retrieved the file '{}' from bucket '{}'",
        &file_name, &bucket_name
    );

    Ok(SuccessResponse {
        body: format!(
            "the lambda has successfully completed the rung '{}'",
            "filename" //filename
        ),
    })
}
