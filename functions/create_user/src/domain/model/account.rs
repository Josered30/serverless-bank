pub struct Account {
    pub user_id: String,
    pub amount: f64,
}

impl Account {
    pub fn new(user_id: String, amount: f64) -> Self {
        Account {
            user_id,
            amount,
        }
    }
}