pub mod db;
pub mod err;
pub mod response;

use std::sync::Arc;

use milvus::client::Client;
use qdrant_client::client::QdrantClient;

use crate::conf::AppConfig;

use crate::middleware::snowflake;

// 状态共享
pub struct AppState {
    pub milvus: Arc<Client>,
    pub confg: Arc<AppConfig>,
}

// 初始化db
pub async fn init_db(conf: AppConfig) -> AppState {
    // 初始化milvus
    let milvus_db = db::init_milvus_client(conf.milvus.clone()).await;


    //初始化snowfork
    snowflake::init_snowflak();

    AppState {
        milvus: Arc::new(milvus_db),
        confg: Arc::new(conf),
    }
}

// 获取 milvus state
pub fn get_milvus(state: &AppState) -> Arc<milvus::client::Client> {
    state.milvus.clone()
}


