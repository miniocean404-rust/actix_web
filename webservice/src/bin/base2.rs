use std::io;

use actix_web::{App, HttpServer};

#[path = "./base2_handle/config.rs"]
mod config;

#[path = "./base2_handle/example.rs"]
mod example;

use crate::config::example_route;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || App::new().configure(example_route);

    println!("服务运行在 http://127.0.0.1:8080");

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
