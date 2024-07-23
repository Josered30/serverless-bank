use tokio::sync::Mutex;

use crate::{
    adapters::repositories::user_credits_repository,
    domain::{
        commands::add_credits_cmd::AddCreditsCmd, errors::api_error::ApiError,
        ports::user_credits_repository::UserCreditsRepository,
    },
};

pub struct AddCreditsCmdHandler {
    user_credits_repository: Mutex<Box<dyn UserCreditsRepository>>,
}

impl AddCreditsCmdHandler {
    pub fn new(user_credits_repository: Mutex<Box<dyn UserCreditsRepository>>) -> Self {
        Self {
            user_credits_repository,
        }
    }

    pub async fn execute(&self, add_credits_cmd: AddCreditsCmd) -> Result<(), ApiError> {
        let user_credits_repository_guard = self.user_credits_repository.lock().await;

        user_credits_repository_guard
            .add_credits(add_credits_cmd.user, add_credits_cmd.amount)
            .await?;

        return Ok(());
    }
}
