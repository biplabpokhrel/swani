// use actix_web::{ web, Error, HttpResponse};
// use futures::Future;
// use futures;

// #[macro_use]
// extern crate actix_web;

use models::account::Account;

pub fn create_account(_new_account: actix_web::web::Json<Account>)
-> impl futures::Future<Item = actix_web::HttpResponse, Error = actix_web::Error> {
    futures::future::ok(actix_web::HttpResponse::Ok().finish())
}