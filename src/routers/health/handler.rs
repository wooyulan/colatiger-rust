use crate::common::resp::RespVO;
pub async fn health_check() -> RespVO<String> {
    RespVO::ok(String::from("ok"))
}
