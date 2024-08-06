use entrypoints::create_user::create_user;
use lambda_http::{run, service_fn, Error};

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

    let service = service_fn(create_user);
    run(service).await?;

    return Ok(());
}
