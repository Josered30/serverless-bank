pub struct AddCreditsCmd {
    pub user: String,
    pub amount: f64,
}

impl AddCreditsCmd {
    pub fn new(user: String, amount: f64) -> Self {
        return AddCreditsCmd { user, amount };
    }
}
