use serde::Deserialize;

#[derive(Deserialize)]
pub struct ImgEmbedStruct {
    pub imgs: Vec<String>
}

