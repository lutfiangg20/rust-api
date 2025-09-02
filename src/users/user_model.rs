use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct UserWithPassword {
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(
        email(message = "Email is not valid"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,

    #[validate(length(min = 1, message = "Phone is required"))]
    pub phone: String,
}
