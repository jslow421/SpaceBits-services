use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    let page_data = get_page_data_as_string().await?;

    Ok(())
}
async fn get_page_data_as_string() -> Result<String, Error> {
    let text = reqwest::get("https://www.nasa.gov/astronauts")
        .await?
        .text()
        .await?;

    Ok(text)
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

// Tests
#[cfg(test)]
mod tests {
    use std::io::empty;

    use super::*;
    use lambda_http::{http::StatusCode, Request, Response};
    use lambda_runtime::Context;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_page_data() {
        let result = get_page_data_as_string().await;
        assert!(result.unwrap().len() > 0);
    }
}
