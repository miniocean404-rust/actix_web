use actix_web::{get, web, HttpResponse, Responder};

pub async fn example_fn() -> impl Responder {
    HttpResponse::Ok().json("例子响应")
}

#[get("/{id}/{name}/index.html")]
async fn restful(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    format!("Hello {}! id:{}", name, id)
}
