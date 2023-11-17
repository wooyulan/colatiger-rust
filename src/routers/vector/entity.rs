use serde::Deserialize;


//请求参数
#[derive(Clone,Deserialize)]
pub struct ImgEmbedReq {
    pub img: String,
    pub biz_no: i64,
}
