//！数据库初始化

use milvus::client::Client;

use crate::config::MilvusConfig;

pub async fn init_milvus_client(conf: MilvusConfig) -> Client {
    println!("milvus连接信息:{}",conf.address);
    let client = match Client::new(conf.address).await {
        Ok(client) =>{
            println!("Connection to the database is successful!");
            client
        }
        Err(e) =>{
            println!("Failed to connect to the database: {:?}", e);
            panic!("Failed to connect to the database for milvus")
        }
    };
    client
}


