use axum::Router;
use axum::routing::get;
use crate::routers::auth::handler::register;




// 向量路由
pub fn auth_router() -> Router {


    Router::new()
        .route("/reg", get(register))
}
