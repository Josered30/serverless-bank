use std::error::Error;

use uuid::Uuid;

use crate::domain::{
    commands::request_payment_cmd::RequestPaymentCmd,
    model::{event_type::EventType, transaction::Transaction},
    ports::event_repository::EventRepository,
};

pub struct RequestPaymentCmdHandlerOutput {
    pub source: String,
    pub id: i32,
}

pub struct RequestPaymentCmdHandler {
    event_repository: Box<dyn EventRepository<Transaction>>,
}

impl RequestPaymentCmdHandler {
    pub fn new(event_repository: Box<dyn EventRepository<Transaction>>) -> Self {
        Self { event_repository }
    }

    pub async fn execute(
        &self,
        request_payment_cmd: RequestPaymentCmd,
    ) -> Result<RequestPaymentCmdHandlerOutput, Box<dyn Error>> {
        let id = 1;
        let source = Uuid::new_v4().to_string();
        let time = chrono::Utc::now().timestamp_millis();

        let transaction = Transaction::new(
            source.clone(),
            id,
            time,
            request_payment_cmd.user_id,
            request_payment_cmd.amount,
            EventType::RequestPayment.to_string(),
        );

        tracing::debug!("Generating event");

        self.event_repository.save_event(transaction).await?;
        tracing::info!("Event saved: {} - {}", &source, id);

        return Ok(RequestPaymentCmdHandlerOutput { source, id });
    }
}
