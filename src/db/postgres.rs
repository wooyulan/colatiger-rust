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

    let db = Database::connect(opt).await.unwrap();
    DB.set(db).unwrap()
}