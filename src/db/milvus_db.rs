use milvus::client::Client;
use milvus::collection::{Collection, ParamValue, SearchOption};
use milvus::data::FieldColumn;
use milvus::error::Error;
use milvus::index::MetricType;
use milvus::value::ValueVec;
use once_cell::sync::OnceCell;
use snafu::{whatever, Whatever};
use crate::common::conf::AppConfig;
use crate::db::snowflake;

pub static MILVUS_CLIENT: OnceCell<Client> = OnceCell::new();

const DEFAULT_VEC_FIELD: &str = "embed";
const DEFAULT_KEY_FIELD: &str = "id";
const DEFAULT_BIZ_FIELD: &str = "biz_no";

pub async fn init_milvus_client(){
    let conf = AppConfig::get_milvus_conf();
    let client = match Client::new(conf.address ).await {
        Ok(client) => {
            tracing::info!("Connection to the milvus is successful!");
            client
        }
        Err(e) => {
            tracing::error!("Failed to connect to the milvus: {:?}", e);
            panic!("Failed to connect to the database for milvus")
        }
    };
    let _= MILVUS_CLIENT.set(client).is_ok();
}
pub fn get_milvus_client() -> Option<&'static Client> {
    MILVUS_CLIENT.get()
}



// 获取collection
pub async fn get_collection(collection_name: &str) -> Result<Collection, Error> {
    let milvus = get_milvus_client().unwrap();
    let collection = milvus
        .get_collection(collection_name)
        .await;
   collection
}

// 持久化数据
pub async fn insert_data(collection_name: &str, data: Vec<Vec<f32>>, biz_no:i64) -> Result<bool,Whatever> {
    let collection = get_collection( collection_name).await.unwrap();
    // 构建向量存储信息
    let schema: &milvus::schema::FieldSchema = collection.schema().get_field(DEFAULT_VEC_FIELD).unwrap();
    let schema_id: &milvus::schema::FieldSchema = collection.schema().get_field(DEFAULT_KEY_FIELD).unwrap();
    let schema_biz_id: &milvus::schema::FieldSchema = collection.schema().get_field(DEFAULT_BIZ_FIELD).unwrap();

    let mut fields_data = Vec::new();

    for temp in data {
        let id = snowflake::next_id();
        let id_colum = FieldColumn::new(schema_id, vec![id]);
        let embed_column = FieldColumn::new(schema, temp, );
        let biz_column=FieldColumn::new(schema_biz_id,vec![biz_no]);
        fields_data.push(id_colum);
        fields_data.push(embed_column);
        fields_data.push(biz_column);
    }
    let ok =  match collection.insert(fields_data, None).await {
        Ok(_) => {
            tracing::info!("milvus插入数据成功");
            true
        }
        Err(err) => {
            tracing::info!("milvus插入数据异常:{:?}",err);
            whatever!("milvus插入数据异常")
        }
    };
    Ok(ok)
}


// 查询数据
pub async fn search_data(collection_name: &str,data:Vec<f32>) -> Result<Vec<i64>,Whatever>{
    let collection = get_collection(collection_name).await.unwrap();
    let mut option = SearchOption::default();
    option.add_param("nprobe", ParamValue!(16));
    let r = match collection.search(vec![data.into()], DEFAULT_VEC_FIELD, 5, MetricType::L2, vec![DEFAULT_BIZ_FIELD],   &option,).await{
        Ok(r) =>{
            let temp = r.get(0).unwrap();
            let result = match temp.field.get(0).unwrap().value.clone() {
                ValueVec::Long(i) => i,
                _ => vec![]
            };
            result
        },
        Err(e) =>{
            tracing::error!("查询异常:{:?}",e);
            whatever!("查询向量库异常")
        }
    };
    Ok(r)
}