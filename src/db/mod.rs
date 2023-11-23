use milvus_db::init_milvus_client;
use snowflake::init_snowflak;
use s3::init_s3;
use postgres::init_postgres;

pub mod milvus_db;
pub mod snowflake;
pub mod s3;
pub mod postgres;
pub mod redis;


pub async fn init_db() {
    // 单例 milvus
    init_milvus_client().await;

    // 雪花
    init_snowflak();

    // 连接S3
    init_s3().await;

    // 连接pgsql
    init_postgres().await;

    // 连接redis
    // init_redis().await;

}
