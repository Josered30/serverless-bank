use std::sync::Arc;

use aws_config::BehaviorVersion;
use lambda_runtime::{Error, LambdaEvent};
use serde::{Deserialize, Serialize};

use crate::{
    adapters::repositories::dyn_transaction_repository::DynTransactionRepository,
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
) -> Result<ExecutePaymentResponse, Error> {
    let execute_payment_cmd =
        ExecutePaymentCmd::new(lambda_event.payload.source, lambda_event.payload.id);

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let dynamodb_client = Arc::new(aws_sdk_dynamodb::Client::new(&config));

    let transaction_repository = Box::new(DynTransactionRepository::new(dynamodb_client.clone()));
    let execute_payment_cmd_handler = ExecutePaymentCmdHandler::new(transaction_repository);

    let result = execute_payment_cmd_handler
        .execute(execute_payment_cmd)
        .await
        .map_err(|error| Error::from(error.to_string()))?;

    return Ok(ExecutePaymentResponse {
        source: result.source,
        id: result.id,
    });
}
