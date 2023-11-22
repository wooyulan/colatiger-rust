use reqwest::header::HeaderMap;
use std::collections::HashMap;



// 发起post请求
pub async fn post(url: &str,data: String) -> Result<HashMap<String, String>, reqwest::Error> {
    // post 请求要创建client
    let client = reqwest::Client::new();
    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // 组装请求参数
    let mut params = HashMap::new();
    params.insert("filePath",data.clone());
    tracing::info!("请求参数{:?}", params);
    // 发起post请求并返回
    Ok(client
        .post(url)
        .headers(headers)
        .json(&params)
        .send()
        .await?
        .json::<HashMap<String,String>>()
        .await?)
}