use async_trait::async_trait;

use crate::domain::errors::repository_error::RepositoryError;

#[async_trait]
pub trait EventRepository<T> {
    async fn save_event(&self, event: T) -> Result<(), RepositoryError>;
    async fn get_events(&self, source: String) -> Result<Vec<T>, RepositoryError>;
}
