pub struct Credentials {
    pub username: String,
    pub pasword: String,
}

impl Credentials {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            pasword: password.to_string(),
        }
    }
}
