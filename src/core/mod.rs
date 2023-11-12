pub mod response;
pub mod  db;

use std::sync::Arc;

use milvus::client::Client;

use crate::conf::AppConfig;

// 状态共享
pub struct AppState {
    pub milvus: Arc<Client>,
    pub confg: Arc<AppConfig>,
}

// 初始化db
pub async fn init_db(conf: AppConfig) -> AppState {
    // 初始化milvus
    let milvus_db = db::init_milvus_client(conf.milvus.clone()).await;
   
    AppState {
        milvus: Arc::new(milvus_db),
        confg: Arc::new(conf)
    }
}
