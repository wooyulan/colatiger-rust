mod conf;
mod core;
mod routers;
use dotenv::dotenv;
use crate::conf::AppConfig;
pub use core::response::Result;

#[tokio::main]
async fn main() {
    
    dotenv().ok();

    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // 加载配置
    let conf = AppConfig::new().unwrap();

    // 初始化中间件
    let app_state =  core::init_db(conf.clone()).await;
 
    //加载路由
    let app = routers::load_router(app_state);

    tracing::info!(
        "{} 启动成功... 当前版本 {},监听地址 {}",
        conf.web.name,
        conf.web.version,
        conf.web.addr
    );

    axum::Server::bind(&conf.web.addr.clone().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
