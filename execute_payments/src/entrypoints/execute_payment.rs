use lambda_runtime::{tower::BoxError, LambdaEvent};
use serde::{Deserialize, Serialize};

use crate::domain::{
    command_handlers::execute_payment_cmd_handler::ExecutePaymentCmdHandler,
    commands::execute_payment_cmd::ExecutePaymentCmd,
};

#[derive(Debug, Deserialize)]
pub struct ExecutePaymentRequest {
    user_id: String,
    amount: f64,
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
        ExecutePaymentCmd::new(&lambda_event.payload.user_id, lambda_event.payload.amount);

    let execute_payment_cmd_handler = ExecutePaymentCmdHandler::new().await;

    let result = execute_payment_cmd_handler
        .execute(execute_payment_cmd)
        .await
        .map_err(|error| BoxError::from(error.to_string()))?;

    return Ok(ExecutePaymentResponse {
        source: result.source,
        id: result.id,
    });
}
