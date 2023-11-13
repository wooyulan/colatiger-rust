use crate::snowflake;

// 存储向量
pub struct Vector {
    pub id:i64,
    pub embed: Vec<f32>, 
    pub biz_no: i64
}



impl Vector {
    pub fn build(embed:Vec<f32>) -> Self {
        Self {
            id : snowflake::next_id(),
            biz_no:snowflake::next_id(),
            embed:embed,
        }
    }
}

