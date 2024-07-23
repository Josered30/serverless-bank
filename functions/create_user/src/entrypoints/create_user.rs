use std::sync::Arc;

use aws_config::BehaviorVersion;
use lambda_http::{http::StatusCode, Body, Error, IntoResponse, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

use crate::{
    adapters::repositories::{
        user_credits_repository::DynUserCreditsRepository, user_repository::DynUserRepository,
    },
    domain::{
        command_handlers::create_user_cmd_handler::CreateUserCmdHandler,
        commands::create_user_cmd::CreateUserCmd,
        ports::{user_credits_repository::UserCreditsRepository, user_repository::UserRepository},
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    email: String,
    #[serde(rename(deserialize = "firstName"))]
    first_name: String,
    #[serde(rename(deserialize = "lastName"))]
    last_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    id: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserError {
    message: String,
}

pub async fn create_user(request: Request) -> Result<Response<Body>, Error> {
    let str_body = std::str::from_utf8(request.body()).expect("invalid utf-8 sequence");

    let body = match serde_json::from_str::<CreateUserRequest>(str_body) {
        Ok(item) => item,
        Err(err) => {
            let resp = (
                StatusCode::BAD_REQUEST,
                json!(CreateUserError {
                    message: err.to_string(),
                }),
            )
                .into_response();

            return Ok(resp.await);
        }
    };

    let create_user_cmd = CreateUserCmd::new(body.email, body.first_name, body.last_name);

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let dynamodb_client = Arc::new(aws_sdk_dynamodb::Client::new(&config));

    let user_repository = Box::new(DynUserRepository::new(dynamodb_client.clone()));
    let user_credits_repository = Box::new(DynUserCreditsRepository::new(dynamodb_client.clone()));

    let create_user_cmd_handler = CreateUserCmdHandler::new(
        Mutex::new(user_repository as Box<dyn UserRepository>),
        Mutex::new(user_credits_repository as Box<dyn UserCreditsRepository>),
    );

    let result = create_user_cmd_handler.execute(create_user_cmd).await;

    let response = match result {
        Ok(id) => (StatusCode::OK, json!(CreateUserResponse { id })).into_response(),
        Err(error) => (
            StatusCode::from_u16(error.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            json!(CreateUserError {
                message: error.message,
            }),
        )
            .into_response(),
    };

    return Ok(response.await);
}
