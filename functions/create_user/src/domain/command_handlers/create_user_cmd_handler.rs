use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::{
    commands::create_user_cmd::CreateUserCmd,
    errors::api_error::ApiError,
    model::user::User,
    ports::{user_credits_repository::UserCreditsRepository, user_repository::UserRepository},
};

pub struct CreateUserCmdHandler {
    user_repository: Mutex<Box<dyn UserRepository>>,
    user_credits_repository: Mutex<Box<dyn UserCreditsRepository>>,
}

impl CreateUserCmdHandler {
    pub fn new(
        user_repository: Mutex<Box<dyn UserRepository>>,
        user_credits_repository: Mutex<Box<dyn UserCreditsRepository>>,
    ) -> Self {
        Self {
            user_repository,
            user_credits_repository,
        }
    }

    pub async fn execute(&self, create_user_cmd: CreateUserCmd) -> Result<String, ApiError> {
        let user_repository_guard = self.user_repository.lock().await;

        let id = Uuid::new_v4().to_string();
        let user = User::new(
            id.clone(),
            create_user_cmd.email,
            create_user_cmd.first_name,
            create_user_cmd.last_name,
        );

        user_repository_guard.save_user(user).await?;

        let user_credits_repository_guard = self.user_credits_repository.lock().await;

        user_credits_repository_guard
            .save_user_credits(id.clone(), 100.0)
            .await?;

        return Ok(id);
    }
}
