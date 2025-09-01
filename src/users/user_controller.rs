use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    common::model::WebResponse,
    users::{
        user_model::{CreateUser, User},
        user_service,
    },
};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello wolrd")
}

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
    let create_user = CreateUser {
        name: body.name.to_owned(),
        email: body.email.to_owned(),
        password: body.password.to_owned(),
    };
    println!("{:?}", create_user);

    let user = user_service::create_user(create_user).await;
    let response: WebResponse<String> = WebResponse {
        data: user,
        message: "success".to_string(),
    };

    HttpResponse::Ok().json(response)
}
