use actix_web::{Scope, web};

pub mod product_controller;
pub mod product_model;
pub mod product_repository;
pub mod product_service;

pub fn scope() -> Scope {
    let scope = web::scope("/products")
        .service(product_controller::products)
        .service(product_controller::create_product);

    scope
}
