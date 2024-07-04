pub struct ExecutePaymentCmd {
    pub source: String,
    pub id: i32,
}

impl ExecutePaymentCmd {
    pub fn new(source: String, id: i32) -> Self {
        return ExecutePaymentCmd {
            source,
            id,
        };
    }
}
