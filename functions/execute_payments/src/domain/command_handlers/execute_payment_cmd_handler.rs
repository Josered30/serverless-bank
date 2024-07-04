use std::error::Error;

use uuid::Uuid;

use crate::{
    adapters::repositories::transaction_repository::TransactionRepository,
    domain::{
        commands::execute_payment_cmd::ExecutePaymentCmd,
        errors::command_handler_error::CommandHandlerError,
        model::{event_type::EventType, transaction::Transaction},
        ports::event_repository::EventRepository,
    },
};

pub struct ExecutePaymentCmdHandlerOutput {
    pub source: String,
    pub id: i32,
}

pub struct ExecutePaymentCmdHandler<'a> {
    transaction_repository: &'a TransactionRepository<'a>,
}

impl<'a> ExecutePaymentCmdHandler<'a> {
    pub fn new(transaction_repository: &'a TransactionRepository) -> Self {
        Self {
            transaction_repository,
        }
    }

    pub async fn execute(
        &self,
        execute_payment_cmd: ExecutePaymentCmd,
    ) -> Result<ExecutePaymentCmdHandlerOutput, Box<dyn Error>> {
        let id = 1;
        let source = Uuid::new_v4().to_string();

        let last_transactions = self
            .transaction_repository
            .get_events(execute_payment_cmd.source)
            .await?;

        let Some(last_transaction) = last_transactions.first() else {
            return Err(CommandHandlerError::Error(
                "Last transaction not found".to_string(),
            ))?;
        };

        let transaction = Transaction::new(
            source.clone(),
            last_transaction.id + 1,
            last_transaction.user_id.to_string(),
            last_transaction.amount,
            EventType::ExecutePayment.to_string(),
        );

        self.transaction_repository.save_event(transaction).await?;

        return Ok(ExecutePaymentCmdHandlerOutput { source, id });
    }
}
