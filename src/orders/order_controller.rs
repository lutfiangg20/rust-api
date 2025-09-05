use actix_web::{HttpResponse, Responder, get, post, web};
use validator::Validate;

use crate::{
    common::model::WebResponse,
    orders::{
        order_model::{CreateOrder, Order},
        order_service,
    },
};

#[get("")]
pub async fn orders() -> impl Responder {
    let orders = order_service::get_all_orders().await;
    let response: WebResponse<Vec<Order>> = WebResponse {
        data: orders,
        message: "success".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("")]
pub async fn create_order(body: web::Json<CreateOrder>) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    };
    let order = order_service::create_order(body.into_inner()).await;
    let response: WebResponse<String> = WebResponse {
        data: order,
        message: "success".to_string(),
    };

    HttpResponse::Ok().json(response)
}
