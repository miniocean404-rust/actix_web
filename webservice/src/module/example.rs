use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub async fn example_fn() -> impl Responder {
    HttpResponse::Ok().body("例子响应")
}

#[get("/{id}/{name}/index.html")]
async fn restful(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    format!("Hello {}! id:{}", name, id)
}

#[derive(Deserialize)]
struct Request {
    id: u64,
}

#[derive(Serialize)]
struct Response {
    id: u64,
}

#[get("/get/query")]
async fn get_query(web::Query(info): web::Query<Request>) -> impl Responder {
    format!("参数 {} ", info.id)
}

#[post("/post/form")]
async fn get_post_form(form: web::Form<Request>) -> impl Responder {
    HttpResponse::Ok().json(form.id)
}

#[post("/post/json")]
async fn get_post_json(json: web::Json<Request>) -> impl Responder {
    let res = Response { id: json.id };

    HttpResponse::Ok().json(web::Json(res))
}
