use actix_web::{Scope, web};

pub mod category_controller;
pub mod category_model;
pub mod category_repository;
pub mod category_service;

pub fn scope() -> Scope {
    let scope = web::scope("/categories")
        .service(category_controller::categories)
        .service(category_controller::create_category);

    scope
}
