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

then we should do something

```bash
cargo lambda build
cargo lambda deploy
# This should parse the route proc macro and configure AWS API Gateway
cargo infra deploy 
```

```rust
let router = Router {
    routes: [
        Router {
            method:HTTPMethod.Get,
            path: "/some/path/{variable}/foo"
        }
    ]
}

let aws_config = aws_config::load_from_env().await;
AWSAPIGatewayConfigurer::new(aws_config).configure(router).await;
```