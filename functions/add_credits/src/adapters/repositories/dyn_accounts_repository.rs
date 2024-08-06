use std::{env, sync::Arc};

use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;

use crate::domain::{
    errors::repository_error::RepositoryError,
    ports::accounts_repository::AccountsRepository,
};

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
    async fn add_credits(&self, user: String, amount: f64) -> Result<(), RepositoryError> {
        let result = match self
            .dynamodb_client
            .update_item()
            .table_name(self.accounts_table_name.to_string())
            .key("user", AttributeValue::S(user))
            .update_expression("ADD #amount :amount")
            .expression_attribute_values(":amount", AttributeValue::N(amount.to_string()))
            .expression_attribute_names("#amount", "amount")
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Repository error: {:?}", error);
                Err(RepositoryError::Error("Error updating item".to_string()))
            }
        };

        return result;
    }
}
