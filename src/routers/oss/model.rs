use serde::{Deserialize, Serialize};

// 文件上传返回
#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct OssVo {
    pub priview_url: String,
    pub key: String
}
