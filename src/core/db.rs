//！数据库初始化

use milvus::client::Client;
use crate::config::MilvusConfig;

pub async fn init_milvus_client(conf: MilvusConfig) -> Client {
    tracing::debug!("milvus连接信息:{}",conf.address);
    let client = match Client::new(conf.address).await {
        Ok(client) =>{
            tracing::info!("Connection to the milvus is successful!");
            client
        }
        Err(e) =>{
            tracing::debug!("Failed to connect to the milvus: {:?}", e);
            panic!("Failed to connect to the database for milvus")
        }
    };
    client
}


