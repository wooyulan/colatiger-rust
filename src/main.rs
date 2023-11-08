
mod config;
mod routers;
mod core;

pub use core::response::Result;




#[tokio::main]
async fn main() {
    // 加载配置

    //加载路由
    let app = routers::load_router();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}