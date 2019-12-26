#[macro_use]
extern crate serde_derive;
extern crate actix_web;

use std::env;
use actix_web::{HttpServer, HttpRequest, App, Responder, middleware, web};

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

async fn index(_req: HttpRequest) -> impl Responder {
    let host_name = get_from_env("HOSTNAME");
    web::Json(Response {
        hostname: host_name,
        status: String::from("OK"),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    HttpServer::new(||
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
    )
    .bind("0.0.0.0:9999")?
    .run()
    .await
}
