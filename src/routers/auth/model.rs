use mll_axum_utils::middleware::jwt::JwtToken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Validate, Serialize, Deserialize)]
struct User {
    uid: u64,
    // 数据验证
    #[validate(length(min = 3, max = 24, message = "用户名长度必须在3-24之间"))]
    name: String,
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Claims {
    exp: u64,
    user: User,
}


impl JwtToken for Claims {
    const SECRET: &'static str = "new_key";
    const DURATION: u64 = 60 * 60 * 24; // token 有效期持续 1 天
}

impl Claims {
    fn new(user: User) -> Self {
        Self {
            exp: Self::expiration(),
            user,
        }
    }
    fn get_uid(user: User) -> u64 {
        user.uid
    }

}