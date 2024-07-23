use std::{collections::HashMap, env, sync::Arc};

use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;

use crate::domain::{
    errors::repository_error::RepositoryError,
    ports::user_credits_repository::UserCreditsRepository,
};

pub struct DynUserCreditsRepository {
    user_credits_table_name: String,
    dynamodb_client: Arc<aws_sdk_dynamodb::Client>,
}

impl DynUserCreditsRepository {
    pub fn new(dynamodb_client: Arc<aws_sdk_dynamodb::Client>) -> Self {
        let user_credits_table_name = match env::var("USER_CREDITS_TABLE_NAME") {
            Ok(var) => var,
            Err(_) => "TABLE_NAME".to_owned(),
        };

        Self {
            dynamodb_client,
            user_credits_table_name,
        }
    }
}

#[async_trait]
impl UserCreditsRepository for DynUserCreditsRepository {
    async fn save_user_credits(&self, user: String, amount: f64) -> Result<(), RepositoryError> {
        let mut item = HashMap::<String, AttributeValue>::new();

        item.insert("user".to_string(), AttributeValue::S(user));
        item.insert("amount".to_string(), AttributeValue::N(amount.to_string()));

        let result = match self
            .dynamodb_client
            .put_item()
            .table_name(self.user_credits_table_name.to_string())
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
