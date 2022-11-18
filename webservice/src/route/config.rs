use actix_web::web::{self};

use crate::module::example::{example_fn, restful, use_post};

// 配置局部路由
pub fn example_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/example", web::get().to(example_fn));

    cfg.service(use_post);
    cfg.service(restful);
    // cfg.service(use_post);
}
