use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub price: i32,
    pub category_id: i32,
}
