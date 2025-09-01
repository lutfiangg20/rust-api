use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    categories::{
        category_model::{CreateCategory, Product},
        category_service,
    },
    common::model::WebResponse,
};

#[get("")]
pub async fn categories() -> impl Responder {
    let categories = category_service::get_all_categories().await;
    let response: WebResponse<Vec<Product>> = WebResponse {
        data: categories,
        message: "success".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("")]
pub async fn create_category(body: web::Json<CreateCategory>) -> impl Responder {
    let create_category = CreateCategory {
        name: body.name.to_owned(),
    };
    println!("{:?}", create_category);

    let user = category_service::create_category(create_category).await;
    let response: WebResponse<String> = WebResponse {
        data: user,
        message: "success".to_string(),
    };

    HttpResponse::Ok().json(response)
}
