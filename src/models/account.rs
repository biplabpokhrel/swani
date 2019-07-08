// use serde::{Deserialize, Serialize};

// #[macro_use]
// extern crate serde;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Account {
    id: Option<i64>,
    username: Option<i64>,
    password: Option<String>,
    user_type: Option<String>,
}