use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::entities::oss::Model;

// 文件上传返回
#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct OssVo {
    pub preview_url: String,
    pub key: i64,
    pub created_at: NaiveDateTime,
    pub file_size: i32,
}

impl OssVo {
    pub fn convert(obj:Model) -> Self {
        Self { 
            preview_url: obj.oss_path,
            key: obj.key_name, 
            created_at: obj.created_at, 
            file_size: obj.file_size 
        }
    }

    pub fn empty() -> Self {
        Self {        
            preview_url: "".to_string(),
            key: 0,
            file_size:0,
            created_at: Default::default(),
        }
    }

}



/// 文件查询
#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct OssQuery{
    pub key: Vec<i64>,
}

