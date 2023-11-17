use axum::extract::Multipart;
use axum::Json;
use axum::response::IntoResponse;
use crate::common::resp::RespVO;
use crate::routers::oss::model::OssVo;
use crate::routers::oss::service;


// 文件上传
pub async fn file_upload(multipart: Multipart) -> impl IntoResponse {
    let oss_vo = service::file_upload(multipart).await.unwrap();
    return Json(RespVO::<OssVo>::ok(oss_vo));
}





