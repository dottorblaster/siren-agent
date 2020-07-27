#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate systemstat;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
use std::env;

mod system;

#[derive(Serialize)]
struct Response {
    hostname: String,
    status: String,
    memory: String,
    load_avg: String,
    uptime: String,
    boot_time: String,
}

fn get_from_env(name: &str) -> String {
    match env::var(name) {
        Ok(value) => value,
        Err(_) => String::from("unset"),
    }
}

async fn index(_req: HttpRequest) -> impl Responder {
    let sys_instance = system::create_instance();
    let host_name = get_from_env("HOSTNAME");
    web::Json(Response {
        hostname: host_name,
        status: String::from("OK"),
        memory: system::memory(&sys_instance),
        load_avg: system::load_avg(&sys_instance),
        uptime: system::uptime(&sys_instance),
        boot_time: system::boot_time(&sys_instance),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
    })
    .bind("0.0.0.0:9999")?
    .run()
    .await
}
