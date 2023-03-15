use crate::config::{AppState, GLOBAL_YAML_CONFIG};
use crate::entity::prelude::Users;
use actix_web::error::ErrorUnauthorized;
use actix_web::{post, web, HttpResponse, Responder};
use brave_utils::jwt::jwt::TokenData;
use sea_orm::EntityTrait;

/*全表查询*/
#[post("/users")]
pub async fn get_users(
    data: web::Data<AppState>,
    token: web::ReqData<TokenData>,
) -> impl Responder {
    let auth = token.auth.clone();

    //只有是超级管理员才能访问
    if auth
        == GLOBAL_YAML_CONFIG
            .authority
            .get_authority_config()
            .super_admin
            .unwrap()
    {
        let db = &data.conn;
        let data = Users::find()
            .into_json()
            .all(db)
            .await
            .expect("Could not find Users");
        HttpResponse::Ok().json(serde_json::json!({"status": "success","data":data}))
    } else {
        ErrorUnauthorized("Lack of authority").into()
    }
}
