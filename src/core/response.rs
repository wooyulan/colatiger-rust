//! 自定义响应状态码
//! 
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Result<T:Serialize> {
   pub error_code: i32,
   pub status: String,
   pub message: String,
   pub data: Option<T>,
}


impl<T> Result<T> 
    where 
        T: Serialize
    {
        pub fn new(error_code: i32,status: String,message:String,data: Option<T>) -> Self {
            Self{error_code,status,message,data}
        }

        pub fn ok(data: Option<T>) -> Self {
            Self::new(200,"ok".to_string(),"success".to_string(),data)
        }

        pub fn fail(error_code:i32,message:String){
            Self::new(error_code,"fail".to_string(), message, None);
        }
    }



