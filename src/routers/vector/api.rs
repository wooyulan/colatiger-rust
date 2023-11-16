use std::string::ToString;
use axum::Router;
use axum::routing::{get, post};
use crate::routers::init::handle_router;


use crate::routers::vector::handler::{img2vector, search_imgs};

pub const VERSION: &str =  "/api/v1";

// 图片向量化
fn emb_img() -> Router {
    //构建注册路由
    handle_router(VERSION.to_string()+"/embed/img", post(img2vector))
}

// * 搜图
fn search_img() -> Router {
    handle_router(VERSION.to_string()+"/img/search",get(search_imgs))
}


// 向量路由
pub fn vector_router() -> Router {
    Router::new()
        .merge(emb_img())
        .merge(search_img())
}
