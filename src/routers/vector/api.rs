use axum::Router;
use axum::routing::{get, post};
use crate::routers::init::handle_router;


use crate::routers::vector::handler::{img2vector, search_imgs};

// 图片向量化
pub fn emb_img() -> Router {
    //构建注册路由
    handle_router("/embed/img", post(img2vector))
}

// * 搜图
pub fn search_img() -> Router {
    handle_router("/img/search",get(search_imgs))
}