use std::collections::HashMap;
use crate::common::resp::RespVO;
use axum::extract::Query;
use crate::routers::oss;
use crate::routers::oss::model::OssQuery;
use crate::routers::vector::entity::ImgEmbedReq;
use crate::routers::vector::service;
use axum::Json;


// 图片向量化
pub async fn img2vector(Json(req): Json<ImgEmbedReq>,
) -> RespVO<bool> {
    if req.img.is_empty() {
        return RespVO::fail_info("图片不能为空");
    }
    // 开始embeding
     match service::embed(req.to_owned()).await {
        Ok(r) => RespVO::ok(r),
        Err(e)=>{
            tracing::error!("img2vector:{:?}",e);
            RespVO::fail_info("向量化失败")
        }
    }
}


// 查询图片
pub async fn search_imgs(Query(params): Query<HashMap<String, String>>) -> RespVO<Vec<String>> {
    let query = params.get("keyword").unwrap();
    if query.is_empty() {
        return RespVO::fail_info("查询参数不能为空");
    }

    // 查询向量库
    let obj = match service::search(query.as_str()).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("img2vector:{:?}",e);
            return RespVO::fail_info("图片向量化失败")
        }
    };

    // 查询图片
    let result = oss::service::file_query(OssQuery{key:obj}).await;
    match result {
        Ok(x) =>  RespVO::ok(x),
        Err(e) => {
            tracing::error!("文件查询异常,e:{}",e);
            RespVO::fail_info("文件查询失败")
        },
    }

}