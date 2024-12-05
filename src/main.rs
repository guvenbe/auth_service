use auth_servicex::Credentials;
fn main() {
    let creds = Credentials::new("guvenb", "password");
    auth_servicex::authenticate(creds);
}
