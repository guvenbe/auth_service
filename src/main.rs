use auth_service::Credentials;
fn main() {
    let creds = Credentials::new("guvenb", "password");
    auth_service::authenticate(creds);
}
