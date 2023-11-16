use config::{Config, File};
use serde::Deserialize;
use std::env;
use once_cell::sync::OnceCell;



// Web配置
#[derive(Debug, Clone, Deserialize)]
pub struct WebConfig {
    /// Web服务监听地址
    pub name: String,
    pub version: String,
    pub addr: String,
}

/// 向量库
#[derive(Debug, Clone, Deserialize)]
pub struct MilvusConfig {
    pub address: String,
}

/// Redis 配置
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    /// 连接字符串
    pub dsn: String,
}

/// 项目配置
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub web: WebConfig,
    pub redis: RedisConfig,
    pub milvus: MilvusConfig,
}


// 从配置文件中读取配置
pub fn init_config() -> AppConfig {
    let run_mode = env::var("mode").unwrap_or_else(|_| "dev".into());

    tracing::info!("start loading {} conf..", run_mode);

    let config = Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
        .build()
        .unwrap()
        .try_deserialize().unwrap();
    config
}

pub static APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

pub fn get() -> Option<&'static AppConfig> {
    APP_CONFIG.get()
}




