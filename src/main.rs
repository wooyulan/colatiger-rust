
use dotenv;
use colatiger::common::conf::{APP_CONFIG, init_config};
use colatiger::routers;
use colatiger::db;


#[tokio::main]
async fn main() {
    //初始化环境变量
    dotenv::dotenv().ok();

    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // // 加载配置
    let conf = APP_CONFIG.get_or_init(|| {init_config()});

    // 初始化数据库
    db::init_db().await;

    //加载路由
    let app = routers::init::routers();

    // middleware::snowflake::next_id();


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


