
mod config;
mod routers;
mod core;

use crate::config::AppConfig;
pub use core::response::Result;







#[tokio::main]
async fn main() {
     // 初始化日志
    // 加载配置
    let conf: &AppConfig = AppConfig::get();

    // 初始化db
    let app_state = core::init_db(conf);


    //加载路由
    let app = routers::load_router(app_state.await);


    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("{} 启动成功... 当前版本 {}", conf.web.name, conf.web.version);
}
