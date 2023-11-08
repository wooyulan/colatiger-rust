
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
    
    // 初始化DB
  

   

    //加载路由
    let app = routers::load_router();


    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

        tracing::info!("{} 启动成功... 当前版本 {}", conf.web.name, conf.web.version);
}
