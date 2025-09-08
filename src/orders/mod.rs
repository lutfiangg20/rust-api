use actix_web::web;

pub mod order_controller;
pub mod order_model;
pub mod order_repository;
pub mod order_repository_test;
pub mod order_service;

pub fn scope() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/orders")
        .service(order_controller::orders)
        .service(order_controller::orders_by_id)
        .service(order_controller::create_order)
    // .wrap(from_fn(auth::auth_service::guard))
    // .service(user_controller::users)
}
