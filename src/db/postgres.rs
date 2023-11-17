use std::time::Duration;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::log;
use crate::common::conf::AppConfig;


pub static DB: OnceCell<DatabaseConnection> = OnceCell::new();
pub async fn init_postgres(){
    let conf = AppConfig::get_db();
    let mut opt = ConnectOptions::new(conf.dns);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info); // Setting default PostgreSQL schema

    let db =match Database::connect(opt).await {
        Ok(db) => db,
        Err(e)=>{
            tracing::error!("Failed to connect to the postgres: {:?}", e);
            panic!("Failed to connect to the database for postgres")
        }
    };
    tracing::info!("Connection to the postgreql is successful!");
    DB.set(db).unwrap()
}

pub fn get_conn() -> Option<&'static DatabaseConnection> { DB.get() }

