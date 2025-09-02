use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCategory {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}
