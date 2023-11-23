use axum::Router;
use axum::routing::{get, post};

use crate::routers::vector::handler::{img2vector, search_imgs};


// 向量路由
pub fn vector_router() -> Router {
    Router::new()
        .route("/embed/img", post(img2vector))
        .route("/img/search",get(search_imgs))
}
