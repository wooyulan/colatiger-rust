use std::io;
use anyhow::Result;
use axum::extract::Multipart;
use crate::db::s3;
use futures::TryStreamExt;
use sea_orm::{Condition, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use tokio_util::io::StreamReader;
use oss::Entity as OSS;
use crate::common::conf::AppConfig;
use crate::db::postgres::DB;
use crate::entities::oss;
use crate::routers::oss::model::{OssQuery, OssVo};
use crate::entities::oss::Model;
use crate::routers::vector;
use crate::routers::vector::entity::ImgEmbedReq;


pub async fn file_upload(mut multipart: Multipart) -> Result<OssVo> {
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
        OSS::insert(active_oss).exec(DB.get().unwrap()).await.unwrap();

        // 构造返回体
        let vo = OssVo {
            priview_url: format!("{}/{}/{}", s3.endpoint, s3.bucket_name, oss_model.oss_path),
            key: oss_model.key_name,
        };

        // 如果是图片 调用embeding
        if content_type.starts_with("image") {
            let thread_vo = vo.clone();
            tokio::spawn(async move {
                vector::service::embed(ImgEmbedReq {
                    img: thread_vo.priview_url,
                    biz_no: thread_vo.key,
                }).await.unwrap()
            }).await.unwrap();
        }

        return Ok(vo);
    };
    Ok(OssVo { priview_url: "".to_string(), key: 0 })
}

pub async fn file_query(query: OssQuery) -> Result<Vec<String>> {
    let objs = OSS::find()
        .filter(
            Condition::all().add(oss::Column::KeyName.is_in(query.key))
        ).all(DB.get().unwrap()).await?;

    let s3 = AppConfig::get_s3_conf();
    let mut file_array = Vec::<String>::new();

    for obj in objs {
        file_array.push(format!("{}/{}/{}", s3.endpoint, s3.bucket_name, obj.oss_path))
    }
    Ok(file_array)
}