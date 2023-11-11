pub mod response;
pub mod  db;

use milvus::client::Client;

use crate::config::AppConfig;

// 状态共享
#[derive(Clone)]
pub struct AppState {
    pub milvus: Client,
}

// 初始化db
pub async fn init_db(conf: &AppConfig) -> AppState {
    // 初始化milvus
    let milvus_db = db::init_milvus_client(conf.milvus.clone()).await;
   
   
    AppState {
        milvus:milvus_db,
    }
}
