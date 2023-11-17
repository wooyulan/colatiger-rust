use reqwest::header::HeaderMap;
use std::collections::HashMap;




// 发起post请求
pub async fn post(url: &str,data: HashMap<&str, Vec<&str>>) -> Result<HashMap<String, Vec<Vec<f32>>>, reqwest::Error> {
    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    tracing::info!("请求参数{:?}", data);
    // 发起post请求并返回
    Ok(client
        .post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, Vec<Vec<f32>>>>()
        .await?)
}