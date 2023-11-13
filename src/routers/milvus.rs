use crate::core::err::Error;
use crate::core::get_milvus;
use crate::core::response::JsonResult;
use crate::{core::AppState, Result};
use axum::Extension;
use axum::{response::IntoResponse, Json};
use rand::prelude::*;
use std::sync::Arc;
use crate::Response;
use milvus::{collection::Collection, data::FieldColumn};

// 验证collection是否存在
pub async fn test_milvus(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let collection_name = "test";
    let milvus = get_milvus(&state);
    let result = milvus.has_collection(collection_name).await.unwrap();

    Json(Response::ok(Some(result)))
}

const DIM: i64 = 256;
const DEFAULT_VEC_FIELD: &str = "embed";

// 插入图片向量
pub async fn insert_pic_vector(Extension(state): Extension<Arc<AppState>>) -> Result<JsonResult<&'static str>> {
    // 向量
    let mut embed_data = Vec::<f32>::new();
    // 主键
    // 业务id

    for _ in 1..=DIM * 1 {
        let mut rng = rand::thread_rng();
        let embed = rng.gen();
        embed_data.push(embed);
    }

    // 获取要操作的集合
    let collection = get_collection(&state, "test").await.map_err(Error::from)?;

    // 构建向量存储信息
    let schema: &milvus::schema::FieldSchema = collection.schema().get_field(DEFAULT_VEC_FIELD).unwrap();
    let embed_column = FieldColumn::new(schema, embed_data);


    let mut field_columns = Vec::<FieldColumn>::new();
    field_columns.push(embed_column);

  

    collection.insert(field_columns, None).await.unwrap();
    collection.flush().await.unwrap();

   // collection.load(1).await.unwrap();

    Ok(Response::ok(Some("ok")).to_json())

}


// 获取collection
pub async fn get_collection(state: &AppState, collection_name: &str) -> Result<Collection> {
    let milvus = get_milvus(&state);
    let collection = milvus.get_collection(collection_name).await.map_err(Error::from)?;
    Ok(collection)
}
