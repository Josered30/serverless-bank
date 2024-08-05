use std::{collections::HashMap, env, sync::Arc};

use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;

use crate::domain::{
    errors::repository_error::RepositoryError, model::user::User,
    ports::user_repository::UserRepository,
};

pub struct DynUserRepository {
    users_table_name: String,
    dynamodb_client: Arc<aws_sdk_dynamodb::Client>,
}

impl DynUserRepository {
    pub fn new(dynamodb_client: Arc<aws_sdk_dynamodb::Client>) -> Self {
        let users_table_name = env::var("USERS_TABLE_NAME").unwrap_or_else(|_| "TABLE_NAME".to_owned());
        Self {
            dynamodb_client,
            users_table_name,
        }
    }
}

#[async_trait]
impl UserRepository for DynUserRepository {
    async fn save_user(&self, user: User) -> Result<(), RepositoryError> {
        let mut item = HashMap::<String, AttributeValue>::new();

        item.insert("id".to_string(), AttributeValue::S(user.id));
        item.insert("email".to_string(), AttributeValue::S(user.email));
        item.insert("first_name".to_string(), AttributeValue::S(user.first_name));
        item.insert("last_name".to_string(), AttributeValue::S(user.last_name));

        let result = match self
            .dynamodb_client
            .put_item()
            .table_name(self.users_table_name.to_string())
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
