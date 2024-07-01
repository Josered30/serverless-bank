use lambda_http::{http::StatusCode, Error, IntoResponse, Request, RequestPayloadExt, Response};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::domain::{
    command_handlers::execute_payment_cmd_handler::{self, ExecutePaymentCmdHandler},
    commands::execute_payment_cmd::ExecutePaymentCmd,
};

#[derive(Debug, Deserialize)]
struct ExecutePaymentRequest {
    user_id: String,
    amount: f64,
}

pub async fn execute_payment(event: Request) -> Result<impl IntoResponse, Error> {
    let request_body = event.payload::<ExecutePaymentRequest>()?;

    let Some(execute_payment_request) = request_body else {
        let response = Response::builder()
            .header("Content-Type", "application/json")
            .status(StatusCode::BAD_REQUEST)
            .body(json!(Value::Null).to_string())
            .map_err(Box::new)?;

        return Ok(response);
    };

    let execute_payment_cmd = ExecutePaymentCmd::new(
        &execute_payment_request.user_id,
        execute_payment_request.amount,
    );

    let execute_payment_cmd_handler = ExecutePaymentCmdHandler::new().await;

    execute_payment_cmd_handler
        .execute(execute_payment_cmd)
        .await;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!(Value::Null).to_string())
        .map_err(Box::new)?;

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    return Ok(response);
}
