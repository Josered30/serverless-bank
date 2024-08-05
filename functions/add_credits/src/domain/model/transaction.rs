#[derive(Debug)]
pub struct Transaction {
    pub source: String,
    pub id: i32,
    pub time: i64,
    pub user_id: String,
    pub amount: f64,
    pub event_type: String,
}

impl Transaction {
    pub fn new(source: String, id: i32, time: i64, user_id: String, amount: f64, event_type: String) -> Self {
        Transaction {
            source,
            id,
            time,
            user_id,
            amount,
            event_type,
        }
    }
}
