use serde::Deserialize;


//请求参数
#[derive(Clone,Deserialize)]
pub struct ImgEmbedReq {
    pub imgs: Vec<String>
}
