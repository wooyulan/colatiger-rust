use std::collections::HashMap;
use crate::common::resp::RespVO;
use axum::{response::IntoResponse, Json};
use axum::extract::Query;
use crate::routers::vector::entity::ImgEmbedReq;
use crate::routers::vector::service;

// 图片向量化
pub async fn img2vector(Json(req): Json<ImgEmbedReq>,
) -> impl IntoResponse {

    if req.img.is_empty() {
        return Json(RespVO::<String>::fail_info("图片不能为空"));
    }
    // 开始embeding
    let result = match service::embed(req.to_owned()).await {
        Ok(_) =>{
            Json(RespVO::ok_empty())
        }
        Err(e)=>{
            tracing::error!("img2vector:{:?}",e);
            Json(RespVO::<String>::fail_info("图片向量化失败"))
        }
    };
    result
}


// 查询图片
pub async fn search_imgs(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let query = params.get("search").unwrap();
    if query.is_empty() {
        return Json(RespVO::fail_info("查询参数不能为空"));
    }

    let result =match service::search(query.as_str()).await {
        Ok(r) => {
            Json(RespVO::ok(r))
        }
        Err(e) => {
            tracing::error!("img2vector:{:?}",e);
            Json(RespVO::fail_info("图片向量化失败"))
        }
    };
    result
}