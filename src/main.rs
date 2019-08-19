#[macro_use]
extern crate serde_derive;
extern crate actix_web;

use std::env;
use actix_web::{HttpServer, App, HttpRequest, Responder, web};

#[derive(Serialize)]
struct Response {
    hostname: String,
    status: String,
}

fn get_from_env(name: &str) -> String {
    match env::var(name) {
        Ok(value) => value,
        Err(_) => String::from("unset"),
    }
}

fn index(_req: HttpRequest) -> impl Responder {
    let host_name = get_from_env("HOSTNAME");
    web::Json(Response {
        hostname: host_name,
        status: String::from("OK"),
    })
}
fn main() {
    HttpServer::new(||{
        App::new().route("/", web::get().to(index))
    })
    .bind("0.0.0.0:9999")
    .expect("Cannot bind to port 9999")
    .run()
    .unwrap();
}
