use super::handles::teacher_fn;
use actix_web::web;

pub fn teacher_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/teacher", web::get().to(teacher_fn));
}
