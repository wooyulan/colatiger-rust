use serde::Deserialize;

// 图片向量
#[derive(Deserialize)]
pub struct ImgEmbedReq {
    pub imgs: Vec<String>
}


