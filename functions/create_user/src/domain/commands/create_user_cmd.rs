pub struct CreateUserCmd {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl CreateUserCmd {
    pub fn new(email: String, first_name: String, last_name: String) -> Self {
        return CreateUserCmd {
            email,
            first_name,
            last_name,
        };
    }
}
