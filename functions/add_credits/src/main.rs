use entrypoints::add_credits::add_credits;
use lambda_http::{run, service_fn, Error};

mod adapters;
mod domain;
mod entrypoints;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let service = service_fn(add_credits);
    run(service).await?;

    return Ok(());
}
