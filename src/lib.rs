#![allow(dead_code, unused_variables)]
use rand::prelude::*;

mod auth_utilsx;
mod database;

pub use auth_utilsx::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = thread_rng().gen_range(100..500);
    print!("timeout: {}", timeout);
    if let Status::Connected = database::connect_to_database() {
        auth_utilsx::login(creds);
    }
}
