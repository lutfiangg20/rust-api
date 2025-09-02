use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(FromRow, Serialize, Deserialize, Debug, Validate, Clone)]
pub struct Login {
    #[validate(length(min = 1, message = "email is required"))]
    pub email: String,
    #[validate(length(min = 1, message = "password is required"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
