The idea is that you only need to add the route mappings for web service using the `route` attribute macro to your AWS lambda functions. That's it. `cargo-infra-build` will build, deploy and configure AWS API gateway for you. No need to deal with Terraform, CloudFront, Serverless or any YAML file.

## Example

```rust
use http::Response;
use lambda_http::{run, http::StatusCode, service_fn, Error, IntoResponse, Request, RequestExt};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
#[route(GET, "/some/path/{variable}/foo")]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    run(service_fn(function_handler)).await
}

pub async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let body = event.payload::<MyPayload>()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({
            "message": "Hello World",
            "payload": body,
          }).to_string())
        .map_err(Box::new)?;

    Ok(response)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyPayload {
    pub prop1: String,
    pub prop2: String,
}
```

then we should be able to build and deploy

```bash
cargo infra-builder deploy
```
