use serde::Deserialize;
use config::{Config, File, ConfigError};
use std::env;

// Web配置
#[derive(Debug, Clone,Deserialize)]
pub struct WebConfig {
    /// Web服务监听地址
    pub name: String,
    pub version: String,
    pub addr: String,
}

/// 向量库
#[derive(Debug, Clone,Deserialize)]
pub struct MilvusConfig {
    pub address: String,
}

/// Redis 配置
#[derive(Debug,Clone, Deserialize)]
pub struct RedisConfig {
    /// 连接字符串
    pub dsn: String,
}


/// 项目配置
#[derive(Debug,Clone)]
#[derive(Deserialize)]
pub struct AppConfig {
    pub web: WebConfig,
    pub redis: RedisConfig,
    pub milvus: MilvusConfig,
}


impl AppConfig {
    // 从配置文件中读取配置
    pub fn new() -> Result<Self,ConfigError>{
     
        let run_mode = env::var("mode").unwrap_or_else(|_| "dev".into());
       
        tracing::info!("start loading {} conf..",run_mode);

        Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(
            File::with_name(&format!("config/{}", run_mode))
                .required(false),
        )
        .build().unwrap()
        .try_deserialize()
    }
}





// impl Default for AppConfig {
//     fn default() -> Self {
//         let file_path = "config.toml";
//         let mut file = match File::open(file_path) {
//             Ok(f) => f,
//             Err(e) => panic!("no such file {} exception:{}", file_path, e)
//         };

//         let mut str_val = String::new();
//         match file.read_to_string(&mut str_val) {
//             Ok(s) => s,
//             Err(e) => panic!("Error Reading file: {}", e)
//         };
//         toml::from_str(&str_val).expect("Parsing the configuration file failed")
//     }
// }

// impl AppConfig {
//     pub fn get<'a>() -> &'a Self {
//         // 给静态变量延迟赋值的宏
//         lazy_static! {
//             static ref CACHE: AppConfig = AppConfig::default();
//         }
//         &CACHE
//     }
// }