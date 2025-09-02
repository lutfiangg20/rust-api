use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    auth::{auth_model::Login, auth_service},
    common::model::WebResponse,
};

#[post("")]
pub async fn login(body: web::Json<Login>) -> impl Responder {
    let login_data = body.into_inner();
    let loggedin = auth_service::login(login_data.clone()).await;
    if !loggedin {
        return HttpResponse::Unauthorized().finish();
    }

    match auth_service::generate_token(&login_data.email).await {
        Ok(token) => {
            let response: WebResponse<String> = WebResponse {
                data: token,
                message: "success".to_string(),
            };

            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().body("token error"),
    }
}
