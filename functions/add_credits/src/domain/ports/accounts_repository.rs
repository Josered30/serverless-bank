use async_trait::async_trait;

use crate::domain::errors::repository_error::RepositoryError;

#[async_trait]
pub trait AccountsRepository: Send {
    async fn add_credits(&self, user: String, amount: f64) -> Result<(), RepositoryError>;
}
