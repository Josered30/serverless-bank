pub struct Transaction {
    pub source: String,
    pub id: i32,
    pub user_id: String,
    pub amount: f64,
    pub event_type: String,
}

impl Transaction {
    pub fn new(source: String, id: i32, user_id: String, amount: f64, event_type: String) -> Self {
        Transaction {
            source,
            id,
            user_id,
            amount,
            event_type,
        }
    }
}
