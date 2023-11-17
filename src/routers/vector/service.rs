use std::collections::HashMap;
use std::error::Error;
use crate::db::milvus_db;
use crate::routers::vector::entity::ImgEmbedReq;
use crate::routers::vector::http_client::post;


const URL_IMG: &str = "http://82.156.138.158:8838/open/api/img2embed";
const URL_TEXT: &str = "http://82.156.138.158:8838/open/api/text2embed";

// embeding
pub async fn embed(req: ImgEmbedReq) -> Result<bool, Box<dyn Error>> {
    // 构建参数
    let params = build_img_param(& req.img);
    // 发送请求
    let res_data = post(URL_IMG,params).await.unwrap();
    let data = match res_data.get("result") {
        Some(v) => v,
        None => {
            panic!("调用向量模型服务异常")
        }
    };
    // 持久化向量数据
    let insert = milvus_db::insert_data("text_img_poc", data.to_owned(),req.biz_no).await;
    Ok(insert)
}

pub async fn search(query: &str) ->Result<Vec<i64>, Box<dyn Error>> {
    let params = build_text_param(query);
    // 发送请求
    let res_data = post(URL_TEXT,params).await.unwrap();
    let data = match res_data.get("result") {
        Some(v) => v,
        None => {
            panic!("调用向量模型服务异常")
        }
    };

    // 查询数据
    let result = milvus_db::search_data("text_img_poc",data.get(0).unwrap().to_owned()).await;
    Ok(result)
}


pub fn build_img_param<'a>(img: &'a String) -> HashMap<&'a str, Vec<&'a str>> {
    let mut params = HashMap::new();
    let mut vec_img = Vec::new();
    vec_img.push(img.as_str());
    params.insert("imgs", vec_img);
    params
}
pub fn build_text_param<'a>(text:&'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut params = HashMap::new();
    let mut vec_text = Vec::new();
    vec_text.push(text);
    params.insert("text", vec_text);
    params
}
