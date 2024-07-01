pub struct Transaction {
    pub source: String,
    pub id: i32,
    pub user_id: String,
    pub amount: f64,
}

impl Transaction {
    pub fn new(source: String, id: i32, user_id: String, amount: f64) -> Self {
        Transaction {
            source,
            id,
            user_id,
            amount,
        }
    }
}
