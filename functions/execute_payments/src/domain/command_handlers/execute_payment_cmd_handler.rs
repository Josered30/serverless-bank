use std::error::Error;

use crate::domain::{
    commands::execute_payment_cmd::ExecutePaymentCmd,
    errors::command_handler_error::CommandHandlerError,
    model::{event_type::EventType, transaction::Transaction},
    ports::event_repository::EventRepository,
};

pub struct ExecutePaymentCmdHandlerOutput {
    pub source: String,
    pub id: i32,
}

pub struct ExecutePaymentCmdHandler {
    transaction_repository: Box<dyn EventRepository<Transaction>>,
}

impl ExecutePaymentCmdHandler {
    pub fn new(transaction_repository: Box<dyn EventRepository<Transaction>>) -> Self {
        Self {
            transaction_repository,
        }
    }

    pub async fn execute(
        &self,
        execute_payment_cmd: ExecutePaymentCmd,
    ) -> Result<ExecutePaymentCmdHandlerOutput, Box<dyn Error>> {
        let last_transactions = self
            .transaction_repository
            .get_events(execute_payment_cmd.source)
            .await?;

        let Some(last_transaction) = last_transactions.first() else {
            return Err(CommandHandlerError::Error(
                "Last transaction not found".to_string(),
            ))?;
        };

        let id = 1;
        let time = chrono::Utc::now().timestamp_millis();
        let source = last_transaction.source.clone();

        let transaction = Transaction::new(
            source.clone(),
            last_transaction.id + 1,
            time,
            last_transaction.user_id.to_string(),
            last_transaction.amount,
            EventType::ExecutePayment.to_string(),
        );

        self.transaction_repository.save_event(transaction).await?;

        return Ok(ExecutePaymentCmdHandlerOutput { source, id });
    }
}
