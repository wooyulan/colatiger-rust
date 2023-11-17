use std::ffi::OsStr;
use chrono::{Datelike, Utc};
use std::path::Path;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use serde::{Deserialize, Serialize};
use crate::db::snowflake;
use sea_orm::EnumIter;
use sea_orm::DerivePrimaryKey;
use sea_orm::DeriveRelation;
use sea_orm::PrimaryKeyTrait;
use sea_orm::EntityTrait;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq,DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "t_oss")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub ori_name:String, //原始文件名

    pub oss_path:String, //oss存储路径

    pub key_name:i64,  // 新名称

    pub is_del: String //是否删除
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


impl Model  {
    pub fn builder(filename: &str) -> Self {
        let now = Utc::now();
        let date_string = format!("{}{}{}", now.year(), now.month(), now.day());
        let path = Path::new(filename);
        let extension = path.extension().unwrap();
        let key = snowflake::next_id();
        let new_name = format!("{}{}{}",key,".",OsStr::new(extension).to_str().unwrap());
        let path = format!("{}/{}",date_string,new_name);
        Self {
            id: snowflake::next_id(),
            ori_name:filename.to_string(),
            key_name: key,
            oss_path: path.to_string(),
            is_del:"N".to_string(),
        }
    }
}