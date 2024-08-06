use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum EventType {
    RequestPayment,
    ExecutePayment,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventType::ExecutePayment => write!(f, "execute-payments.v1"),
            EventType::RequestPayment => write!(f, "request-payments.v1"),
        }
    }
}
