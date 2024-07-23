use async_trait::async_trait;

use crate::domain::{errors::repository_error::RepositoryError, model::user::User};

#[async_trait]
pub trait UserRepository: Send {
    async fn save_user(&self, user: User) -> Result<(), RepositoryError>;
}
