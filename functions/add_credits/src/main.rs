use lambda_runtime::{Error, run, service_fn};
use entrypoints::add_credits::add_credits;

mod adapters;
mod domain;
mod entrypoints;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json()
        .with_max_level(tracing::Level::INFO)
        .with_current_span(false)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .init();

    let service = service_fn(add_credits);
    run(service).await?;

    return Ok(());
}
