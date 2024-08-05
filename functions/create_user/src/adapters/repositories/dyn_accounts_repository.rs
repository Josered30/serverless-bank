use std::{collections::HashMap, env, sync::Arc};

use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;

use crate::domain::{
    errors::repository_error::RepositoryError,
    ports::accounts_repository::AccountsRepository,
};
use crate::domain::model::account::Account;

pub struct DynAccountsRepository {
    accounts_table_name: String,
    dynamodb_client: Arc<aws_sdk_dynamodb::Client>,
}

impl DynAccountsRepository {
    pub fn new(dynamodb_client: Arc<aws_sdk_dynamodb::Client>) -> Self {
        let accounts_table_name = env::var("ACCOUNTS_TABLE_NAME").unwrap_or_else(|_| "TABLE_NAME".to_owned());
        Self {
            dynamodb_client,
            accounts_table_name,
        }
    }
}

#[async_trait]
impl AccountsRepository for DynAccountsRepository {
    async fn save_account(&self, account: Account) -> Result<(), RepositoryError> {
        let mut item = HashMap::<String, AttributeValue>::new();

        item.insert("user".to_string(), AttributeValue::S(account.user_id));
        item.insert("amount".to_string(), AttributeValue::N(account.amount.to_string()));

        let result = match self
            .dynamodb_client
            .put_item()
            .table_name(self.accounts_table_name.to_string())
            .set_item(Some(item))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Repository error: {:?}", error);
                Err(RepositoryError::Error("Error saving item".to_string()))
            }
        };

        return result;
    }
}
