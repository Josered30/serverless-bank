pub struct ExecutePaymentCmd {
    pub user_id: String,
    pub amount: f64,
}

impl ExecutePaymentCmd {
    pub fn new(user_id: &str, amount: f64) -> Self {
        return ExecutePaymentCmd {
            user_id: user_id.to_string(),
            amount,
        };
    }
}
