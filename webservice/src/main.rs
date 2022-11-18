use actix_web::{web, App, HttpServer};
use std::{io, time::Duration};
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

    // 自动启动一些 HTTP workers，默认情况下这个数量等于系统中物理 CPU 的数量。这个数字可以用HttpServer::workers()方法覆盖
    // HttpServer::new(app).workers(4) 指定线程数量
    HttpServer::new(app)
        // Actix Web 保持连接打开以等待后续请求
        .keep_alive(Duration::from_secs(75))
        .bind((LOCAL_HOST, PORT))?
        .run()
        .await
}
