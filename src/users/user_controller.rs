use actix_web::{HttpResponse, Responder, get, post, web};
use validator::Validate;

use crate::{
    common::model::WebResponse,
    users::{
        user_model::{CreateUser, User},
        user_service,
    },
};

#[get("")]
pub async fn users() -> impl Responder {
    let users = user_service::get_all_users().await;
    let response: WebResponse<Vec<User>> = WebResponse {
        data: users,
        message: "success".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("")]
pub async fn create_user(body: web::Json<CreateUser>) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    };
    let user = user_service::create_user(body.into_inner()).await;
    let response: WebResponse<String> = WebResponse {
        data: user,
        message: "success".to_string(),
    };

    HttpResponse::Ok().json(response)
}
