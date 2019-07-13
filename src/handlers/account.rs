// use actix_web::web;
use controllers;

pub fn resources(config: &mut actix_web::web::ServiceConfig) {
    config.service(
        actix_web::web::scope("/accounts")
        .service(
            actix_web::web::resource("")
            .route(actix_web::web::post().to_async(controllers::account::create_account)),
        )
    );
}