#[macro_use]
extern crate serde_derive;
extern crate actix_web;

use std::env;
use actix_web::{server, App, HttpRequest, Responder, Json};

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

fn index(_req: &HttpRequest) -> impl Responder {
    let host_name = get_from_env("HOSTNAME");
    Json(Response {
        hostname: host_name,
        status: String::from("OK"),
    })
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("0.0.0.0:9999")
        .unwrap()
        .run();
}
