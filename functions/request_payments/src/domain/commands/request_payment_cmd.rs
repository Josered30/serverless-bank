pub struct RequestPaymentCmd {
    pub user_id: String,
    pub amount: f64,
}

impl RequestPaymentCmd {
    pub fn new(user_id: String, amount: f64) -> Self {
        return RequestPaymentCmd { user_id, amount };
    }
}
