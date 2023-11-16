use std::io;
use axum::extract::Multipart;
use axum::Json;
use axum::response::IntoResponse;
use futures::TryStreamExt;
use tokio_util::io::StreamReader;
use crate::common;
use crate::common::resp::RespVO;
use crate::db::s3;


// 文件上传
pub async fn file_upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };
        let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        let path = common::file_utils::rename_file(&filename);

        s3::put_file(&path, body_reader).await;
        return Json(RespVO::<String>::ok(path));
    };
    return Json(RespVO::<String>::fail_info("文件失败"));
}





