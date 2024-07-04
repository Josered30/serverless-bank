use aws_config::BehaviorVersion;
use lambda_runtime::{tower::BoxError, LambdaEvent};
use serde::{Deserialize, Serialize};

use crate::{
    adapters::repositories::transaction_repository::TransactionRepository,
    domain::{
        command_handlers::request_payment_cmd_handler::RequestPaymentCmdHandler,
        commands::request_payment_cmd::RequestPaymentCmd,
    },
};

#[derive(Debug, Deserialize)]
pub struct RequestPaymentRequest {
    user_id: String,
    amount: f64,
}

#[derive(Debug, Serialize)]
pub struct RequestPaymentResponse {
    source: String,
    id: i32,
}

pub async fn request_payment(
    lambda_event: LambdaEvent<RequestPaymentRequest>,
) -> Result<RequestPaymentResponse, BoxError> {
    tracing::info!(
        "Init request payment for user {}",
        lambda_event.payload.user_id
    );

    let execute_payment_cmd =
        RequestPaymentCmd::new(lambda_event.payload.user_id, lambda_event.payload.amount);

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let transaction_repository = TransactionRepository::new(&dynamodb_client);
    let execute_payment_cmd_handler = RequestPaymentCmdHandler::new(&transaction_repository);

    let result = execute_payment_cmd_handler
        .execute(execute_payment_cmd)
        .await
        .map_err(|error| BoxError::from(error.to_string()))?;

    return Ok(RequestPaymentResponse {
        source: result.source,
        id: result.id,
    });
}
