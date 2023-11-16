use sea_orm::entity::prelude::*;


pub struct RegistReq {
    pub name: String,
    pub password:String,
}

