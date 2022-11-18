use actix_web::web;

use crate::module::example::{get_post_form, get_post_json, get_query, restful};

// 路由
pub fn set_route(cfg: &mut web::ServiceConfig) {
    cfg.service(restful);
    cfg.service(get_query);
    cfg.service(get_post_form);
    cfg.service(get_post_json);
}
