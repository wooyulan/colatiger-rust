use milvus_db::init_milvus_client;
use snowflake::init_snowflak;

pub mod milvus_db;
pub mod snowflake;


pub async fn init_db() {

    // 单例 milvus
    init_milvus_client().await;

    init_snowflak();


}
