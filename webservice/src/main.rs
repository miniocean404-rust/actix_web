use actix_web::{web, App, HttpServer};
use std::io;
use webservice::{config::example_route, LOCAL_HOST, PORT};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || App::new().service(web::scope("/v1").configure(example_route));

    println!(
        "{}",
        format_args!("服务运行在 http://{}:{}", LOCAL_HOST, PORT)
    );

    HttpServer::new(app).bind((LOCAL_HOST, PORT))?.run().await
}
