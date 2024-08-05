use std::future::Future;
use std::sync::Arc;

use aws_config::BehaviorVersion;
use aws_lambda_events::dynamodb::{Event};
use lambda_runtime::{Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_item};
use tokio::sync::Mutex;
use tokio::task::JoinSet;

use crate::{
    adapters::repositories::dyn_accounts_repository::DynAccountsRepository,
    domain::{
        command_handlers::add_credits_cmd_handler::AddCreditsCmdHandler,
        commands::add_credits_cmd::AddCreditsCmd,
        ports::accounts_repository::AccountsRepository,
    },
};

use crate::domain::errors::base_error::BaseError;
use crate::domain::model::event_type::EventType;


#[derive(Debug, Deserialize, Serialize)]
pub struct DynamoDBTransaction {
    pub source: String,
    pub id: i32,
    pub time: i64,
    pub user_id: String,
    pub amount: f64,
    pub event_type: String,
}

async fn execute_cmd_handler(add_credits_cmd_handler: Arc<AddCreditsCmdHandler>, add_credits_cmd: AddCreditsCmd) -> Result<(), BaseError> {
    let result = add_credits_cmd_handler.execute(add_credits_cmd).await;
    return result;
}

pub async fn add_credits(event: LambdaEvent<Event>) -> Result<(), Error> {
    tracing::info!("Init add credits");

    let add_credits_cmds = event.payload.records
        .into_iter()
        .filter(|record| !record.change.new_image.is_empty())
        .map(|record| {
            tracing::info!("Record: {:?}", record);
            let transaction: DynamoDBTransaction = from_item(record.change.new_image).unwrap();
            return transaction;
        })
        .filter(|transaction| transaction.event_type == EventType::ExecutePayment.to_string())
        .map(|transaction| {
            tracing::info!("Transaction: {:?}", transaction);
            return AddCreditsCmd::new(transaction.user_id, transaction.amount);
        })
        .collect::<Vec<AddCreditsCmd>>();

    if add_credits_cmds.is_empty() {
        return Ok(());
    }

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let dynamodb_client = Arc::new(aws_sdk_dynamodb::Client::new(&config));

    let user_credits_repository = Box::new(DynAccountsRepository::new(dynamodb_client.clone()));
    let add_credits_cmd_handler = Arc::new(AddCreditsCmdHandler::new(Mutex::new(
        user_credits_repository as Box<dyn AccountsRepository>,
    )));

    let mut task_set = JoinSet::new();
    add_credits_cmds.into_iter()
        .for_each(|add_credits_cmd| {
            task_set.spawn(execute_cmd_handler(add_credits_cmd_handler.clone(), add_credits_cmd));
        });

    while let Some(task_result) = task_set.join_next().await {
        match task_result {
            Ok(result) => match result {
                Ok(_) => {}
                Err(error) => tracing::error!("Error: {:?}", error)
            }
            Err(task_error) => tracing::error!("Task error: {:?}", task_error)
        }
    }

    return Ok(());
}
