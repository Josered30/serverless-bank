use tokio::sync::Mutex;

use crate::{
    domain::{
        commands::add_credits_cmd::AddCreditsCmd,
        ports::accounts_repository::AccountsRepository,
    },
};
use crate::domain::errors::base_error::BaseError;

pub struct AddCreditsCmdHandler {
    accounts_repository: Mutex<Box<dyn AccountsRepository>>,
}

impl AddCreditsCmdHandler {
    pub fn new(accounts_repository: Mutex<Box<dyn AccountsRepository>>) -> Self {
        Self {
            accounts_repository,
        }
    }

    pub async fn execute(&self, add_credits_cmd: AddCreditsCmd) -> Result<(), BaseError> {
        let accounts_repository_guard = self.accounts_repository.lock().await;

        accounts_repository_guard
            .add_credits(add_credits_cmd.user, add_credits_cmd.amount)
            .await?;

        return Ok(());
    }
}
