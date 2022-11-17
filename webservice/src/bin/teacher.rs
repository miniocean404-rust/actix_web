use actix_web::{web, App, HttpServer};
use std::{io, sync::Mutex};

#[path = "../state.rs"]
mod state;

#[path = "../router.rs"]
mod route;

#[path = "../handles.rs"]
mod handles;

use route::*;
use state::AppState;

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
