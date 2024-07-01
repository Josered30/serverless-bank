use uuid::Uuid;

use crate::{
    adapters::repositories::transaction_repository::TransactionRepository,
    domain::{
        commands::execute_payment_cmd::ExecutePaymentCmd, model::transaction::Transaction,
        ports::event_repository::EventRepository,
    },
};

pub struct ExecutePaymentCmdHandlerOutput {
    pub source: String,
    pub id: i32,
}

pub struct ExecutePaymentCmdHandler {
    transaction_repository: TransactionRepository,
}

impl ExecutePaymentCmdHandler {
    pub async fn new() -> Self {
        Self {
            transaction_repository: TransactionRepository::new().await,
        }
    }

    pub async fn execute(
        &self,
        execute_payment_cmd: ExecutePaymentCmd,
    ) -> ExecutePaymentCmdHandlerOutput {
        let id = 1;
        let source = Uuid::new_v4().to_string();

        let transaction = Transaction::new(
            source.clone(),
            id,
            execute_payment_cmd.user_id,
            execute_payment_cmd.amount,
        );

        match self.transaction_repository.save_event(transaction).await {
            Ok(_) => (),
            Err(error) => println!("{:?}", error),
        }

        return ExecutePaymentCmdHandlerOutput { source, id };
    }
}
