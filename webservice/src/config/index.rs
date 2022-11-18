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
            // guard 校验是否有对应的请求头
            .guard(guard::Header("Host", "www.rust-lang.org"))
            .to(example_fn),
    );

    // 在资源 /test 下 使用 get 请求 返回 方法不支持
    cfg.service(web::resource("/test").route(web::get().to(HttpResponse::MethodNotAllowed)));
}
