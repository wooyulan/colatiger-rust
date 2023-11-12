use std::sync::Arc;
use crate::{Result,core::AppState};
use axum::{Json,response::IntoResponse};
use axum::Extension;

// 验证collection是否存在
pub async fn test_milvus(
    Extension(state): Extension<Arc<AppState>>,
)-> impl IntoResponse{
   
    let collection_name ="test";

    let result = state.milvus.clone().has_collection(collection_name).await.unwrap();
    
    Json(Result::ok(Some(result)))
}