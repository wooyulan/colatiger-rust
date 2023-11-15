use crate::core::response::JsonResult;
use crate::{Response, snowflake};
use crate::{core::AppState, Result};
use axum::Extension;
use axum::{response::IntoResponse, Json};
use milvus::{collection::Collection, data::FieldColumn};
use snafu::ResultExt;
use std::sync::Arc;
use rand::prelude::*;
use crate::core::get_milvus;
use crate::llm::clip::load_clip;

// 验证collection是否存在
pub async fn test_milvus(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let collection_name = "test";
    let milvus = get_milvus(&state);
    let result = milvus.has_collection(collection_name).await.unwrap();

    Json(Response::ok(Some("ok")))
}

const DIM: i64 = 2;
const DEFAULT_VEC_FIELD: &str = "embed";

// 插入图片向量
pub async fn insert_pic_vector(Extension(state): Extension<Arc<AppState>>) -> Result<JsonResult<&'static str>> {

    // 主键id
    let data = load_clip().await;


    // 获取要操作的集合
    let collection = get_collection(&state, "test01").await?;
    // 构建向量存储信息
    let schema: &milvus::schema::FieldSchema = collection.schema().get_field(DEFAULT_VEC_FIELD).unwrap();
    let schema_id: &milvus::schema::FieldSchema = collection.schema().get_field("id").unwrap();


    // // 组建业务主键
    for temp in data {
        let id = snowflake::next_id();

        let id_colum = FieldColumn::new(
            schema_id, vec![id],
        );

        let embed_column = FieldColumn::new(
            schema,
            temp,
        );

        let ok = match collection.insert(vec![id_colum, embed_column], None).await {
            Ok(v) => {
                println!("{:?}", v);
                collection.flush().await.unwrap();
                collection.load(1).await.unwrap();
            }
            Err(e) => {
                println!("{:?}", e)
            }
        };
    }

    Ok(Response::ok(Some("ok")).to_json())
}

// 获取collection
pub async fn get_collection(state: &AppState, collection_name: &str) -> Result<Collection> {
    let milvus = get_milvus(state);
    let collection = milvus
        .get_collection(collection_name)
        .await
        .with_whatever_context(|_| format!("collection not exist"))?;
    Ok(collection)
}
