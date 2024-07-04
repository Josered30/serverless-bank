use aws_config::BehaviorVersion;
use lambda_runtime::{tower::BoxError, LambdaEvent};
use serde::{Deserialize, Serialize};

use crate::{
    adapters::repositories::transaction_repository::TransactionRepository,
    domain::{
        command_handlers::execute_payment_cmd_handler::ExecutePaymentCmdHandler,
        commands::execute_payment_cmd::ExecutePaymentCmd,
    },
};

#[derive(Debug, Deserialize)]
pub struct ExecutePaymentRequest {
    source: String,
    id: i32,
}

#[derive(Debug, Serialize)]
pub struct ExecutePaymentResponse {
    source: String,
    id: i32,
}

pub async fn execute_payment(
    lambda_event: LambdaEvent<ExecutePaymentRequest>,
) -> Result<ExecutePaymentResponse, BoxError> {
    let execute_payment_cmd =
        ExecutePaymentCmd::new(lambda_event.payload.source, lambda_event.payload.id);

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let transaction_repository = TransactionRepository::new(&dynamodb_client);
    let execute_payment_cmd_handler = ExecutePaymentCmdHandler::new(&transaction_repository);

    let result = execute_payment_cmd_handler
        .execute(execute_payment_cmd)
        .await
        .map_err(|error| BoxError::from(error.to_string()))?;

    return Ok(ExecutePaymentResponse {
        source: result.source,
        id: result.id,
    });
}
