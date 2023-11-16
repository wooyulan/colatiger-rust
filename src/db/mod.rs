use milvus_db::init_milvus_client;
use snowflake::init_snowflak;
use s3::init_s3;

pub mod milvus_db;
pub mod snowflake;
mod s3;


pub async fn init_db() {

    // 单例 milvus
    init_milvus_client().await;

    // 雪花
    init_snowflak();

    init_s3().await;


}
