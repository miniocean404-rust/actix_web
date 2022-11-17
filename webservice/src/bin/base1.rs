use std::io;

use actix_web::{
    get, post,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

struct AppState {
    app_name: String,
}

#[get("/index")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// #[get("/{id}/{name}/index.html")]
// async fn restful(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", name, id)
// }

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(hello)
            .service(index)
            // .service(index1)
            .route("/hey", web::get().to(manual_hello))
    };

    println!("服务运行在 http://127.0.0.1:8080");

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
