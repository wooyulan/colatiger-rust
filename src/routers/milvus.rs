use std::collections::HashMap;
use crate::core::response::JsonResult;
use crate::{llm, Response, snowflake};
use crate::{core::AppState, Result};
use axum::Extension;
use axum::{response::IntoResponse, Json};
use snafu::ResultExt;
use std::sync::Arc;
use axum::extract::Query;
use milvus::index::MetricType;
use milvus::collection::{Collection, ParamValue, SearchOption};
use milvus::data::FieldColumn;
use milvus::value::ValueVec;
use crate::core::get_milvus;
use crate::req::vector::ImgEmbedReq;



const URL_IMG: &str = "http://82.156.138.158:8838/open/api/img2embed";
const URL_TEXT: &str = "http://82.156.138.158:8838/open/api/text2embed";
const DEFAULT_VEC_FIELD: &str = "embed";

// 验证collection是否存在
pub async fn search_pic(Extension(state): Extension<Arc<AppState>>,
                        Query(params): Query<HashMap<String, String>>
) -> Result<JsonResult<Vec<i64>>> {
    let query = params.get("search").unwrap();
    let params = build_param_text(query);

    let result = llm::clip::post(URL_TEXT, params).await.unwrap();
    let data = match result.get("result") {
        Some(v) => v,
        None => {
            return Ok(Response::fail(0,"调用向量模型服务异常").to_json())
        }
    };

    let reust = search_data(&state,data.get(0).unwrap().to_owned()).await;

    Ok(Response::ok(Some(reust)).to_json())
}



// 插入图片向量
pub async fn insert_pic_vector(Extension(state): Extension<Arc<AppState>>,
                               Json( req): Json<ImgEmbedReq>,
) -> Result<JsonResult<&'static str>> {

    let params = build_param(& req.imgs);

    let result = llm::clip::post(URL_IMG, params).await.unwrap();
    let data = match result.get("result") {
        Some(v) => v,
        None => {
            return Ok(Response::fail(0,"调用向量模型服务异常").to_json())
        }
    };
    insert_data(&state, data.to_owned()).await
}


pub fn build_param<'a>(imgs: &'a Vec<String>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut params = HashMap::new();
    let mut vec_img = Vec::new();
    for img in imgs {
        vec_img.push(img.as_str());
    }
    params.insert("imgs", vec_img);
    params
}

pub fn build_param_text<'a>(text:&'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut params = HashMap::new();
    let mut vec_text = Vec::new();
    vec_text.push(text);
    params.insert("text", vec_text);
    params
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

// 插入数据
pub async fn insert_data(state: &AppState, data: Vec<Vec<f32>>) -> Result<JsonResult<&'static str>> {
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
        let _ok = match collection.insert(vec![id_colum, embed_column], None).await {
            Ok(_v) => {
                collection.flush().await.unwrap();
                collection.load(1).await.unwrap();
            }
            Err(e) => {
                tracing::error!("milvus insert err{}",e);
                return Ok(Response::fail(0, "milvus insert fail").to_json());
            }
        };
    }
    Ok(Response::ok(Some("ok")).to_json())
}

// 向量搜索
pub async fn search_data(state: &AppState,data: Vec<f32>) -> Vec<i64> {
    let collection = get_collection(state,"test01").await.unwrap();

    let mut option = SearchOption::default();
    option.add_param("nprobe", ParamValue!(16));

    println!("--=-=-====");

    let r = match collection.search(vec![data.into()], DEFAULT_VEC_FIELD, 5, MetricType::L2, vec!["id"],   &option,).await{
        Ok(r) =>{
            let temp = r.get(0).unwrap();
            let result = match temp.field.get(0).unwrap().value.clone() {
                ValueVec::Long(i) => i,
                _ => vec![]
            };
            result
        },
        Err(e) =>{
            println!("result:{:?}",e);
            panic!("")
        }
    };
    r

}