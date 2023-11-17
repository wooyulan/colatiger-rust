use once_cell::sync::OnceCell;
use s3::bucket::Bucket;
use s3::region::Region;
use s3::creds::Credentials;
use crate::common::conf::AppConfig;
use std:: pin::Pin;
use tokio::io::AsyncRead;

pub static S3: OnceCell<Bucket> = OnceCell::new();

pub async fn init_s3() {

   let s3 = AppConfig::get_s3_conf();
   let bucket_name = s3.bucket_name.as_str();
   let region = Region::Custom { region: "".to_owned(), endpoint: s3.endpoint.to_owned()};

   let credentials = match Credentials::new(Some(s3.access_key.as_str()), Some(s3.secret_access_key.as_str()), None, None, None) {
      Ok(cred) => cred,
      Err(e) => {
         tracing::error!("Failed to connect to the s3: {:?}", e);
         panic!("Failed to connect to the database for s3")
      }
   };

   let bucket = match Bucket::new(bucket_name, region, credentials) {
      Ok(ok) => ok.with_path_style(),
      Err(e) =>{
         tracing::error!("Failed to connect to the s3: {:?}", e);
         panic!("Failed to connect to the database for s3")
      }
   };

   match bucket.list("/".to_string(), Some("/".to_string())).await {
      Ok(obj) =>{
         tracing::info!("connect to the s3 success, bucket size:: {:?}", obj.len());
      }
      Err(e)=>{
         tracing::error!("Failed to connect to the s3: {:?}", e);
         panic!("Failed to connect to the database for s3")
      }
   }
   let _= S3.set(bucket).is_ok();
}

pub fn get_s3_bucket() -> Option<&'static Bucket> { S3.get() }


// 上传文件
pub async fn put_file(filename: &str,mut reader: Pin<&mut (dyn AsyncRead + Send)>){
   let bucket = get_s3_bucket().unwrap();
   let _ =bucket.put_object_stream(&mut reader, &filename).await;
}




