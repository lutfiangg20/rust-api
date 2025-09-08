use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize)]
pub struct Order {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct OrderItem {
    pub product_name: String,
    pub quantity: i32,
    pub price: i32,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct OrderQuery {
    pub id: i32,
    pub name: String,
    pub item_id: Option<i32>,
    pub product_id: Option<i32>,
    pub product_name: Option<String>,
    pub status: String,
    pub quantity: Option<i32>,
    pub price: Option<i32>,
}

// #[derive(Debug, FromRow, Serialize, Clone)]
// pub struct CreateOrderQuery {
//     pub id: i32,
//     pub name: String,
//     pub status: String,
// }

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateOrder {
    #[validate(range(min = 1, message = "user_id is not valid"))]
    pub user_id: i32,
    pub status: String,
    pub items: Vec<CreateOrderItem>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateOrderItem {
    // pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: i32,
}
