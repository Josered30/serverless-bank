use async_trait::async_trait;

use crate::domain::errors::repository_error::RepositoryError;

#[async_trait]
pub trait UserCreditsRepository: Send {
    async fn save_user_credits(&self, user: String, amount: f64) -> Result<(), RepositoryError>;
}
