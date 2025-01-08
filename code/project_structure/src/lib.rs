#![allow(dead_code, unused_variables)]

pub mod auth_utils;
mod database;

use auth_utils::models::Credentials;
use database::{Status, connect_to_database};

pub fn authenticate(cred: Credentials) {
    if let Status::Connected = connect_to_database() {}
}
