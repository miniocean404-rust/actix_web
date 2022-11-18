use actix_web::{
    guard,
    web::{self},
    HttpResponse,
};

use crate::{module::example::example_fn, route::set_route};

// 配置局部路由
pub fn example_route(cfg: &mut web::ServiceConfig) {
    set_route(cfg);

    cfg.route(
        "/example",
        web::get()
            .to(example_fn)
            .guard(guard::Header("Host", "www.rust-lang.org")),
    );
    cfg.service(web::resource("/test").route(web::head().to(HttpResponse::MethodNotAllowed)));
}
