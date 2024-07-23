use std::sync::Arc;

use aws_config::BehaviorVersion;
use lambda_http::{http::StatusCode, Body, Error, IntoResponse, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

use crate::{
    adapters::repositories::user_credits_repository::DynUserCreditsRepository,
    domain::{
        command_handlers::add_credits_cmd_handler::AddCreditsCmdHandler,
        commands::add_credits_cmd::AddCreditsCmd,
        ports::user_credits_repository::UserCreditsRepository,
    },
};

#[derive(Debug, Deserialize)]
pub struct AddCreditsRequest {
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
    amount: f64,
}

#[derive(Debug, Serialize)]
pub struct AddCreditsResponse {}

#[derive(Debug, Serialize)]
pub struct AddCreditsError {
    message: String,
}

pub async fn add_credits(request: Request) -> Result<Response<Body>, Error> {
    let str_body = std::str::from_utf8(request.body()).expect("invalid utf-8 sequence");

    let body = match serde_json::from_str::<AddCreditsRequest>(str_body) {
        Ok(item) => item,
        Err(err) => {
            let resp = (
                StatusCode::BAD_REQUEST,
                json!(AddCreditsError {
                    message: err.to_string(),
                }),
            )
                .into_response();

            return Ok(resp.await);
        }
    };

    let execute_payment_cmd = AddCreditsCmd::new(body.user_id.to_string(), body.amount);

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let dynamodb_client = Arc::new(aws_sdk_dynamodb::Client::new(&config));

    let user_credits_repository = Box::new(DynUserCreditsRepository::new(dynamodb_client.clone()));

    let execute_payment_cmd_handler = AddCreditsCmdHandler::new(Mutex::new(
        user_credits_repository as Box<dyn UserCreditsRepository>,
    ));

    let result = execute_payment_cmd_handler
        .execute(execute_payment_cmd)
        .await;

    let response = match result {
        Ok(_) => (StatusCode::OK, json!(AddCreditsResponse {})).into_response(),
        Err(error) => (
            StatusCode::from_u16(error.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            json!(AddCreditsError {
                message: error.message,
            }),
        )
            .into_response(),
    };

    return Ok(response.await);
}
