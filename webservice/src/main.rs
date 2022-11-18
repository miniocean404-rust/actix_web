use actix_web::{web, App, HttpServer};
use std::io;
use webservice::{index::example_route, static_server::static_server, LOCAL_HOST, PORT};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || {
        // 因为闭包里 这个 configure 返回的是 self 所以 scope 在外部使用的时候 这个闭包就是 FnOnce
        let scope = web::scope("/v1").configure(example_route);

        App::new().service(scope).service(static_server())
    };

    println!(
        "{}",
        format_args!("服务运行在 http://{}:{}", LOCAL_HOST, PORT)
    );

    HttpServer::new(app).bind((LOCAL_HOST, PORT))?.run().await
}
