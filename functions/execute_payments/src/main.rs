use entrypoints::execute_payment::execute_payment;
use lambda_runtime::{run, service_fn, Error};

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

    let service = service_fn(execute_payment);
    let _ = run(service).await;

    return Ok(());
}
