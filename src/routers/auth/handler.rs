use axum::Json;
use axum::response::IntoResponse;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use db::snowflake::next_id;
use crate::{db, entity};
use crate::common::resp::RespVO;
use crate::routers::auth::model::RegistReq;

// 注册
pub async fn register() -> impl IntoResponse{
    let user = entity::users::ActiveModel {
        id: Set(next_id()),
        username:Set("test".to_string()),
        password:Set("123456".to_string()),
        is_del:Set("N".to_string()),
        ..Default::default()
    };
    user.insert(db::postgres::get_conn().unwrap()).await.unwrap();
    Json(RespVO::<String>::ok_empty())
}