use actix_web::{web, App, HttpServer};
use std::{io, sync::Mutex};

#[path = "./base3_handle/module.rs"]
mod module;
use crate::module::*;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let share = web::Data::new(AppState {
        health: "我是健康的".to_string(),
        count: Mutex::new(0),
    });

    let app = move || App::new().app_data(share.clone()).configure(teacher_route);

    println!("服务运行在 http://127.0.0.1:8080");

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
