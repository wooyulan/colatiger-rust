use axum::Router;
use axum::routing::post;
use axum::extract::DefaultBodyLimit;

use crate::routers::init::handle_router;
use crate::routers::oss::handler::file_upload;


pub const VERSION: &str =  "/api/v1";


// 文件上传
pub fn  upload() -> Router {
    handle_router(VERSION.to_string()+"/upload", post(file_upload)).layer(DefaultBodyLimit::max(1024*1024*30))
}


