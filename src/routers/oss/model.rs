use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// 文件上传返回
#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct OssVo {
    pub preview_url: String,
    pub key: i64,
    pub created_at: NaiveDateTime,
    pub file_size: i32,
}

/// 文件查询
#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct OssQuery{
    pub key: Vec<i64>,
}

