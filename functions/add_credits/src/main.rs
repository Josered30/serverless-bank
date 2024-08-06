use entrypoints::add_credits::add_credits;
use lambda_runtime::{run, service_fn, Error};

mod adapters;
mod domain;
mod entrypoints;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::DEBUG)
        .with_current_span(false)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .init();

    let service = service_fn(add_credits);
    run(service).await?;

    return Ok(());
}
