use actix_web::{web, HttpResponse, Responder};

use crate::state::AppState;

pub async fn teacher_fn(state: web::Data<AppState>) -> impl Responder {
    let health = &state.health;

    let mut count = state.count.lock().unwrap();

    let res = format!("{} {} times", health, count);

    *count += 1;

    HttpResponse::Ok().json(&res)
}
