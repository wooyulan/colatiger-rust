use axum::extract::Multipart;
use axum::Json;
use axum::response::IntoResponse;
use crate::common::resp::RespVO;
use crate::routers::oss::model::{OssQuery, OssVo};
use crate::routers::oss::service;


// 文件上传
pub async fn file_upload(multipart: Multipart) -> impl IntoResponse {
    let oss_vo = service::file_upload(multipart).await.unwrap();
    return Json(RespVO::<OssVo>::ok(oss_vo));
}


pub async fn file_query(Json(req): Json<OssQuery>,)-> impl IntoResponse{
    let result = service::file_query(req).await;
    match result {
        Ok(x) => Json(RespVO::<Vec<String>>::ok(x)),
        Err(e) => Json(RespVO::<Vec<String>>::fail_info("查询失败")),
    }
}




