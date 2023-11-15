use reqwest::header::HeaderMap;
use std::collections::HashMap;

const URL: &str = "http://82.156.138.158:8838/open/api/text2embed";



pub async fn load_clip() -> Vec<Vec<f32>> {
    let params = build_param();
    let mut result = post(URL, params).await.unwrap();
    let temp = match result.get("result") {
         Some(v) => v,
         None => return Vec::new(),
    };
    temp.to_owned()

}
    // let mut embed_data = Vec::<f64>::new();
    //
    // for i in temp {
    //     println!("{:?}",i.len());
    //     for l in i {
    //
    //     }
    // }





pub fn build_param() -> HashMap<&'static str, Vec<&'static str>> {
    let mut params = HashMap::new();
    let mut text_arr = Vec::new();
    text_arr.push("你好");
    text_arr.push("你好");
    text_arr.push("你好");
    params.insert("text", text_arr);
    params
}

async fn post(
    url: &str,
    data: HashMap<&str, Vec<&str>>,
) -> Result<HashMap<String, Vec<Vec<f32>>>, reqwest::Error> {
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
