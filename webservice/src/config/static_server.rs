use std::env;

use actix_files::Files;

// 静态服务
pub fn static_server() -> Files {
    // 获取 cargo 默认环境变量 使用 env!()
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    // 设置变量 PUBLIC_PATH="test" cargo run -p webservice --bin webservice
    let _public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    // http://127.0.0.1:8080/v1
    Files::new("/", "./webservice/public").index_file("index.html")

    // 我们允许访问者在' /images '处看到图像的索引
    // Files::new("/images", "static/images/").show_files_listing()
}
