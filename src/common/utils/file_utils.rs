use std::ffi::OsStr;
use std::path::Path;
use chrono::{Datelike, Utc};
use crate::db::snowflake;

pub fn rename_file(filename: &str) -> String {
    let now = Utc::now();
    let date_string = format!("{}{}{}", now.year(), now.month(), now.day());
    let path = Path::new(filename);
    let extension = path.extension().unwrap();
    let id = snowflake::next_id();
    let new_name = format!("{}{}{}",id,".",OsStr::new(extension).to_str().unwrap());
    let path = format!("{}/{}",date_string,new_name);
    path
}