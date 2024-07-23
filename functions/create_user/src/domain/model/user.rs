pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl User {
    pub fn new(id: String, email: String, first_name: String, last_name: String) -> Self {
        User {
            id,
            email,
            first_name,
            last_name,
        }
    }
}
