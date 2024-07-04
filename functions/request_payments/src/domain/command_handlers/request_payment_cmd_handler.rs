use std::error::Error;

use uuid::Uuid;

use crate::{
    adapters::repositories::transaction_repository::TransactionRepository,
    domain::{
        commands::request_payment_cmd::RequestPaymentCmd,
        model::{event_type::EventType, transaction::Transaction},
        ports::event_repository::EventRepository,
    },
};

pub struct RequestPaymentCmdHandlerOutput {
    pub source: String,
    pub id: i32,
}

pub struct RequestPaymentCmdHandler<'a> {
    transaction_repository: &'a TransactionRepository<'a>,
}

impl<'a> RequestPaymentCmdHandler<'a> {
    pub fn new(transaction_repository: &'a TransactionRepository) -> Self {
        Self {
            transaction_repository,
        }
    }

    pub async fn execute(
        &self,
        request_payment_cmd: RequestPaymentCmd,
    ) -> Result<RequestPaymentCmdHandlerOutput, Box<dyn Error>> {
        let id = 1;
        let source = Uuid::new_v4().to_string();

        let transaction = Transaction::new(
            source.clone(),
            id,
            request_payment_cmd.user_id,
            request_payment_cmd.amount,
            EventType::RequestPayment.to_string(),
        );

        tracing::debug!("Generating event");
        
        self.transaction_repository.save_event(transaction).await?;
        tracing::info!("Event saved: {} - {}", &source, id);

        return Ok(RequestPaymentCmdHandlerOutput { source, id });
    }
}
