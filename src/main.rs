use actix_web::{
    web,
    App,
    HttpRequest,
    HttpServer,
    Responder,
    HttpResponse,
    middleware
};

extern crate log;
extern crate actix_web;
extern crate futures;
extern crate serde;
//extern crate serde-derive;

mod handlers;
mod controllers;
mod models;

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    let message = format!("Hello {}!", &name);
    HttpResponse::Ok().json(message)
}


fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info,swani::controllers=info");
    env_logger::init();
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