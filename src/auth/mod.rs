use actix_web::{Scope, web};

mod auth_controller;
mod auth_model;
mod auth_repository;
mod auth_service;

pub fn scope() -> Scope {
    web::scope("/auth").service(auth_controller::login)
}
