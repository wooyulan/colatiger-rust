use once_cell::sync::OnceCell;
use redis::Connection;
use crate::common::conf::AppConfig;

pub static CACHE: OnceCell<Connection> = OnceCell::new();
pub async fn init_redis() {
    let conf = AppConfig::get_redis_conf();

    let client =match redis::Client::open(conf.dsn){
        Ok(client) => client,
        Err(_e) =>{
            panic!("Failed to connect to the database for redis")
        }
    };
    let _ = match client.get_connection() {
        Ok(conn) => {
            tracing::info!("Connection to the redis is successful!");
            CACHE.set(conn)
        },
        Err(_e) => {
            panic!("Failed to connect to the database for redis")
        }
    };

}

pub fn get_cache() -> Option<&'static Connection> { CACHE.get() }