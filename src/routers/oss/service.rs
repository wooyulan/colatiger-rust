use std::io;
use axum::extract::Multipart;
use crate::db::s3;
use futures::TryStreamExt;
use sea_orm::{Condition, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, ActiveModelTrait, PaginatorTrait};
use sea_orm::ActiveValue::Set;
use snafu::{Whatever, whatever};
use tokio_util::io::StreamReader;
use oss::Entity as OSS;
use crate::common::conf::AppConfig;
use crate::common::page::{PageQuery,PageResult};
use crate::db::postgres::DB;
use crate::entities::oss;
use crate::routers::oss::model::{OssQuery, OssVo};
use crate::entities::oss::Model;
use crate::routers::oss::remote;
use crate::routers::vector;
use crate::routers::vector::entity::ImgEmbedReq;


pub async fn file_upload(mut multipart: Multipart) -> Result<OssVo, Whatever> {
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

        // 如果是图片 调用embedding
        if content_type.starts_with("image") {
            // 调用embedding
            let thread_vo = vo.clone();
            tokio::spawn(async move {
                vector::service::embed(ImgEmbedReq {
                    img: thread_vo.priview_url,
                    biz_no: thread_vo.key,
                }).await.unwrap()
            }).await.unwrap();

            let img_url = vo.priview_url.to_owned();
            // 调用图片识别
            _ = match remote::post("http://127.0.0.1:8000/open/v1/img",img_url).await {
                Ok(obj) => {
                    // 更新数据库
                    let mut up_model = oss_model.to_owned().into_active_model();
                    up_model.file_type = Set(obj.get("type").unwrap().to_string());
                    up_model.update(DB.get().unwrap()).await.unwrap();
                },
                Err(e) => {
                    tracing::error!("{:?}",e);
                    whatever!("图拍分类异常")
                }
            };
        }

        return Ok(vo);
    };
    Ok(OssVo { priview_url: "".to_string(), key: 0 })
}

pub async fn file_query(query: OssQuery) -> Result<Vec<String>,Whatever> {
    let objs = match OSS::find()
        .filter(
            Condition::all().add(oss::Column::KeyName.is_in(query.key))
        ).all(DB.get().unwrap()).await{
        Ok(obj) => obj,
        Err(e) =>{
            tracing::error!("文件查询数据库异常:e:{:?}",e);
            whatever!("文件查询数据库异常{:?}",e);
        }
    };
    let s3 = AppConfig::get_s3_conf();
    let mut file_array = Vec::<String>::new();

    for obj in objs {
        file_array.push(format!("{}/{}/{}", s3.endpoint, s3.bucket_name, obj.oss_path))
    }
    Ok(file_array)
}



pub async fn query_page(params:PageQuery) -> Result<PageResult<Model>,Whatever> {
    let page = params.page_no();
    let page_size = params.page_size();

    let paginator = OSS::find().paginate(DB.get().unwrap(),page_size);

    let total = paginator.num_pages().await.unwrap();
    let items = paginator.fetch_page(page).await.unwrap();

    let result = PageResult {
        params: params,
        data:items,
        total:total,
    };

    Ok(result)
}