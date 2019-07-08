use actix_web::{
    web,
    App,
    HttpRequest,
    HttpServer,
    Responder,
    HttpResponse,
    middleware
};

mod handlers;
mod controllers;
mod models;

#[macro_use]
extern crate log;
extern crate actix_web;
extern crate futures;
extern crate serde;

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    let message = format!("Hello {}!", &name);
    HttpResponse::Ok().json(message)
}


fn main() {
    info!("Server started!!!");
    HttpServer::new(|| {
        App::new()
            .configure(handlers::account::resources)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .wrap(middleware::Logger::default())

    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000, I have no idea why")
    .run()
    .unwrap();
}