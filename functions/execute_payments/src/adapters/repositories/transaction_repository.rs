use std::{collections::HashMap, env};

use aws_sdk_dynamodb::types::AttributeValue;

use crate::domain::{
    errors::repository_error::RepositoryError, model::transaction::Transaction,
    ports::event_repository::EventRepository,
};

pub struct TransactionRepository<'a> {
    transaction_table_name: String,
    dynamodb_client: &'a aws_sdk_dynamodb::Client,
}

impl<'a> TransactionRepository<'a> {
    pub fn new(dynamodb_client: &'a aws_sdk_dynamodb::Client) -> Self {
        let transaction_table_name = match env::var("TRANSACTION_EVENT_TABLE_NAME") {
            Ok(var) => var,
            Err(_) => "TABLE_NAME".to_owned(),
        };

        Self {
            dynamodb_client,
            transaction_table_name,
        }
    }
}

impl<'a> EventRepository<Transaction> for TransactionRepository<'a> {
    async fn save_event(&self, transaction: Transaction) -> Result<(), RepositoryError> {
        let mut item = HashMap::<String, AttributeValue>::new();

        item.insert("source".to_string(), AttributeValue::S(transaction.source));

        item.insert(
            "id".to_string(),
            AttributeValue::N(transaction.id.to_string()),
        );

        item.insert(
            "user_id".to_string(),
            AttributeValue::S(transaction.user_id),
        );

        item.insert(
            "amount".to_string(),
            AttributeValue::N(transaction.amount.to_string()),
        );

        let result = match self
            .dynamodb_client
            .put_item()
            .table_name(&self.transaction_table_name)
            .set_item(Some(item))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Repository error: {:?}", error);
                return Err(RepositoryError::Error("Error saving event".to_string()));
            }
        };

        return result;
    }

    async fn get_events(&self, source: String) -> Result<Vec<Transaction>, RepositoryError> {
        let query_output = self
            .dynamodb_client
            .query()
            .table_name(&self.transaction_table_name)
            .expression_attribute_names("#source", "source")
            .expression_attribute_values(":source", AttributeValue::S(source.clone()))
            .key_condition_expression("#source = :source")
            .limit(1)
            .scan_index_forward(false)
            .send()
            .await;

        let result = match query_output {
            Ok(items) => items,
            Err(error) => {
                println!("Repository error: {:?}", error);
                return Err(RepositoryError::Error(format!(
                    "Error retrieving item with source {source}"
                )));
            }
        };

        let Some(items) = result.items else {
            return Ok(Vec::new());
        };

        let result: Vec<Transaction> = items
            .into_iter()
            .map(|item| {
                Transaction::new(
                    item.get("source").unwrap().as_s().unwrap().to_string(),
                    item.get("id")
                        .unwrap()
                        .as_n()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    item.get("user_id").unwrap().as_s().unwrap().to_string(),
                    item.get("amount")
                        .unwrap()
                        .as_n()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    item.get("event_type").unwrap().as_s().unwrap().to_string(),
                )
            })
            .collect();

        return Ok(result);
    }
}
