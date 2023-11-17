use axum::extract::Multipart;
use axum::Json;
use crate::common::resp::RespVO;
use crate::routers::oss::model::{OssQuery, OssVo};
use crate::routers::oss::service;


// 文件上传
pub async fn file_upload(multipart: Multipart) -> RespVO<OssVo> {
    let res = service::file_upload(multipart).await;
    match res {
        Ok(x) => RespVO::ok(x),
        Err(e) => {
            tracing::error!("文件上传异常,e:{}",e);
            RespVO::fail_info("文件上传异常")
        }
    }
}


pub async fn file_query(Json(req): Json<OssQuery>,)-> RespVO<Vec<String>>{
    let result = service::file_query(req).await;
    match result {
        Ok(x) =>  RespVO::ok(x),
        Err(e) => {
            tracing::error!("文件查询异常,e:{}",e);
            RespVO::fail_info("文件查询失败")
        },
    }
}




