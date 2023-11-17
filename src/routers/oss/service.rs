use std::io;
use axum::extract::Multipart;
use crate::db::s3;
use futures::TryStreamExt;
use sea_orm::{EntityTrait, IntoActiveModel};
use tokio_util::io::StreamReader;
use crate::common::conf::AppConfig;
use crate::db::postgres::DB;
use crate::entities::oss;
use crate::routers::oss::model::OssVo;
use crate::entities::oss::Model;
use crate::routers::vector;
use crate::routers::vector::entity::ImgEmbedReq;


pub async fn file_upload(mut multipart: Multipart) -> Result<OssVo, ()> {
    let s3 = AppConfig::get_s3_conf();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };

        //文件类型
        let content_type = field.content_type().unwrap().to_string();

        tracing::info!("文件类型为{:?}",content_type);


        let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        let oss_model = Model::builder(&filename);
        s3::put_file(&oss_model.oss_path, body_reader).await;

        // 存储数据库
        let active_oss = oss_model.to_owned().into_active_model();
        oss::Entity::insert(active_oss).exec(DB.get().unwrap()).await.unwrap();

        // 构造返回体
        let vo = OssVo {
            priview_url: format!("{}/{}/{}", s3.endpoint, s3.bucket_name, oss_model.oss_path),
            key: oss_model.key_name.to_owned(),
        };

        // 如果是图片 调用embeding
        if content_type.starts_with("image") {
            let thread_vo = vo.clone();
            tokio::spawn(async move {
                vector::service::embed(ImgEmbedReq{
                    imgs:vec![thread_vo.priview_url]
                }).await.unwrap()
            }).await.unwrap();
        }

        return Ok(vo);
    };
    Ok(OssVo { priview_url: "".to_string(), key: "".to_string() })
}