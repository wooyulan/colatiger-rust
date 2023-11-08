use serde::Deserialize;
use std::sync::{OnceLock};


// Web配置
#[derive(Debug, Clone)]
#[derive(Deserialize)]
pub struct WebConfig {
    /// Web服务监听地址
    pub addr: String,
}

/// Redis 配置
#[derive(Deserialize)]
#[derive(Debug, Clone)]
pub struct RedisConfig {
    /// 连接字符串
    pub dsn: String,
}


/// 项目配置
#[derive(Debug, Clone)]
#[derive(Deserialize)]
pub struct AppConfig {
    pub web: WebConfig,
    pub redis: RedisConfig,
}


pub fn init_app_config() -> AppConfig {
    let raw_config = std::fs::read_to_string("config.toml")
         .expect("Failed to read config.toml");
     let app_config = toml::from_str::<AppConfig>(&raw_config)
         .expect("Failed to parse config.toml");
     app_config
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

