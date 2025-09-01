use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    common::model::WebResponse,
    products::{
        product_model::{CreateProduct, Product},
        product_service,
    },
};

#[get("")]
pub async fn products() -> impl Responder {
    let products = product_service::get_all_products().await;
    let response: WebResponse<Vec<Product>> = WebResponse {
        data: products,
        message: "success".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("")]
pub async fn create_product(body: web::Json<CreateProduct>) -> impl Responder {
    let create_product = CreateProduct {
        name: body.name.to_owned(),
        price: body.price.to_owned(),
        category_id: body.category_id.to_owned(),
    };
    println!("{:?}", create_product);

    let user = product_service::create_product(create_product).await;
    let response: WebResponse<String> = WebResponse {
        data: user,
        message: "success".to_string(),
    };

    HttpResponse::Ok().json(response)
}
