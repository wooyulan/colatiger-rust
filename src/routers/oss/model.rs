use serde::{Deserialize, Serialize};

// 文件上传返回
#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct OssVo {
    pub priview_url: String,
    pub key: i64
}

/// 文件查询
#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct OssQuery{
    pub key: Vec<i64>,
}

