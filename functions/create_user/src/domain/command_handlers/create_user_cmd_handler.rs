use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::{
    commands::create_user_cmd::CreateUserCmd,
    errors::api_error::ApiError,
    model::user::User,
    ports::{accounts_repository::AccountsRepository, user_repository::UserRepository},
};
use crate::domain::model::account::Account;

pub struct CreateUserCmdHandler {
    user_repository: Mutex<Box<dyn UserRepository>>,
    accounts_repository: Mutex<Box<dyn AccountsRepository>>,
}

impl CreateUserCmdHandler {
    pub fn new(
        user_repository: Mutex<Box<dyn UserRepository>>,
        accounts_repository: Mutex<Box<dyn AccountsRepository>>,
    ) -> Self {
        Self {
            user_repository,
            accounts_repository,
        }
    }

    pub async fn execute(&self, create_user_cmd: CreateUserCmd) -> Result<String, ApiError> {
        let id = Uuid::new_v4().to_string();
        let user = User::new(
            id.clone(),
            create_user_cmd.email,
            create_user_cmd.first_name,
            create_user_cmd.last_name,
        );

        let user_repository_guard = self.user_repository.lock().await;
        user_repository_guard.save_user(user).await?;

        let account = Account::new(id.clone(), 100.0);

        let accounts_repository_guard = self.accounts_repository.lock().await;
        accounts_repository_guard.save_account(account).await?;

        return Ok(id);
    }
}
