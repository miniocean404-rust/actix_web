use std::io;

use actix_web::{
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

// 配置局部路由
pub fn example_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/example", web::get().to(example_fn));
}

async fn example_fn() -> impl Responder {
    HttpResponse::Ok().json("例子响应")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || App::new().configure(example_route);

    println!("服务运行在 http://127.0.0.1:8080");

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
