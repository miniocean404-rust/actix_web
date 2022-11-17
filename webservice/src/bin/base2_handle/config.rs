use super::example::{example_fn, restful};
use actix_web::web::{self};

// 配置局部路由
pub fn example_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/example", web::get().to(example_fn));

    cfg.service(restful);
}
