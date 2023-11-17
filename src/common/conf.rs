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

/// S3 配置
#[derive(Debug, Clone, Deserialize)]
pub struct S3Config {
    pub endpoint: String,
    pub access_key: String,
    pub secret_access_key: String,
    pub bucket_name: String,
    pub use_ssl: bool,
}

///db
#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    pub dns: String,
}


/// 项目配置
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub web: WebConfig,
    pub redis: RedisConfig,
    pub milvus: MilvusConfig,
    pub s3: S3Config,
    pub db: DbConfig,
}


// 从配置文件中读取配置
pub fn init_config() -> AppConfig {
    let run_mode = env::var("env").unwrap_or_else(|_| "default".into());

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

impl AppConfig {
    pub fn get_db() -> DbConfig{
        get().unwrap().db.to_owned()
    }
    pub fn get_milvus_conf()-> MilvusConfig {
       get().unwrap().milvus.to_owned()
    }

    pub fn get_s3_conf()-> S3Config {
        get().unwrap().s3.to_owned()
    }

    pub fn get_web_conf()-> WebConfig  {
        get().unwrap().web.to_owned()
    }
}





