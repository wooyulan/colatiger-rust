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
   let region = Region::Custom { region: "".into(), endpoint: s3.endpoint.into()  };
   let credentials = Credentials::new(Some(s3.access_key.as_str()), Some(s3.secret_access_key.as_str()), None, None, None).unwrap();
   let bucket = Bucket::new(bucket_name, region, credentials).unwrap().with_path_style();

   let _= S3.set(bucket).is_ok();
   tracing::info!("Connection to the s3 is successful!");
}

pub fn get_s3_bucket() -> Option<&'static Bucket> { S3.get() }


// 上传文件
pub async fn put_file(filename: &str,mut reader: Pin<&mut (dyn AsyncRead + Send)>){
   let bucket = get_s3_bucket().unwrap();
   let _ =bucket.put_object_stream(&mut reader, &filename).await;
}




