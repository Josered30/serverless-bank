use async_trait::async_trait;

use crate::domain::errors::repository_error::RepositoryError;
use crate::domain::model::account::Account;

#[async_trait]
pub trait AccountsRepository: Send {
    async fn save_account(&self, account: Account) -> Result<(), RepositoryError>;
}
