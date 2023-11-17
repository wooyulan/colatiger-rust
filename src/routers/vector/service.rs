use std::collections::HashMap;
use snafu::{whatever, Whatever};
use crate::db::milvus_db;
use crate::routers::vector::entity::ImgEmbedReq;
use crate::routers::vector::http_client::post;


const URL_IMG: &str = "http://82.156.138.158:8838/open/api/img2embed";
const URL_TEXT: &str = "http://82.156.138.158:8838/open/api/text2embed";

// embeding
pub async fn embed(req: ImgEmbedReq) -> Result<bool, Whatever> {
    // 构建参数

    let params = build_param(& "imgs",& req.img.as_str());
    // 发送请求
    let res_data = post(URL_IMG,params).await.unwrap();
    let data = match res_data.get("result") {
        Some(v) => v,
        None => {
            whatever!("调用向量模型服务异常")
        }
    };
    // 持久化向量数据
    milvus_db::insert_data("text_img_poc", data.to_owned(),req.biz_no).await
}

pub async fn search(query: &str) ->Result<Vec<i64>, Whatever> {
    let params = build_param(& "text", query);
    // 发送请求
    let res_data = post(URL_TEXT,params).await.unwrap();
    let data = match res_data.get("result") {
        Some(v) => v,
        None => {
            whatever!("调用向量模型服务异常")
        }
    };
    // 查询数据
    milvus_db::search_data("text_img_poc",data.get(0).unwrap().to_owned()).await
}


pub fn build_param<'a>(key:&'a str, data: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut params = HashMap::new();
    let mut vec_img = Vec::new();
    vec_img.push(data);
    params.insert(key, vec_img);
    params
}
