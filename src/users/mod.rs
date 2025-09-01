use actix_web::{Scope, web};

pub mod user_controller;
pub mod user_model;
pub mod user_repository;
pub mod user_service;

pub fn scope() -> Scope {
    let scope = web::scope("/users")
        .service(user_controller::users)
        .service(user_controller::create_user);

    scope
}
