use actix_web::{middleware::from_fn, web};

use crate::auth;

pub mod user_controller;
pub mod user_model;
pub mod user_repository;
pub mod user_service;

pub fn scope() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/users")
        .service(user_controller::create_user)
        .service(
            web::scope("")
                .wrap(from_fn(auth::auth_service::guard))
                .service(user_controller::users),
        )
}
