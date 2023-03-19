use infra_builder::route;
use lambda_http::Error;

#[tokio::main]
#[route(FOO, "/")]
async fn main() -> Result<(), Error> {
    Ok(())
}
