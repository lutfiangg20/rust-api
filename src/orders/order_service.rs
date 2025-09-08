use std::collections::HashMap;

use actix_web::{
    Error,
    error::{ErrorInternalServerError, ErrorNotFound},
};

use crate::orders::{
    order_model::{CreateOrder, Order, OrderItem, OrderQuery},
    order_repository,
};

pub async fn get_all_orders() -> Vec<Order> {
    let order_repo = order_repository::Repo::new()
        .await
        .find_all()
        .await
        .expect("allaala");

    let mut order_map: HashMap<i32, Vec<OrderQuery>> = HashMap::new();

    for order in order_repo {
        order_map.entry(order.id).or_default().push(order);
    }

    let mut data: Vec<Order> = order_map
        .into_iter()
        .map(|(id, items)| {
            let name = items.first().map(|o| o.name.clone()).unwrap_or_default();
            let items = items
                .into_iter()
                .map(|item| OrderItem {
                    product_name: item.product_name.clone().unwrap_or_default(),
                    quantity: item.quantity.unwrap_or_default(),
                    price: item.price.unwrap_or_default(),
                })
                .collect();

            Order {
                id,
                name,
                status: "pending".to_string(),
                items,
            }
        })
        .collect();

    data.sort_by_key(|d| d.id);
    data
}

pub async fn get_order_by_id(id: i32) -> Result<Order, Error> {
    let order_repo = order_repository::Repo::new()
        .await
        .find_by_id(id)
        .await
        .map_err(ErrorInternalServerError)?;

    // cek apakah data kosong
    if order_repo.is_empty() {
        return Err(ErrorNotFound("Order not found"));
    }

    // group by order id
    let mut order_map: HashMap<i32, Vec<OrderQuery>> = HashMap::new();
    for order in order_repo {
        order_map.entry(order.id).or_default().push(order);
    }

    // ambil satu order
    let (order_id, items) = order_map.into_iter().next().unwrap();

    let name = items.first().map(|o| o.name.clone()).unwrap_or_default();
    let status = items.first().map(|o| o.status.clone()).unwrap_or_default();
    let items = items
        .into_iter()
        .map(|item| OrderItem {
            product_name: item.product_name.unwrap_or_default(),
            quantity: item.quantity.unwrap_or_default(),
            price: item.price.unwrap_or_default(),
        })
        .collect();

    Ok(Order {
        id: order_id,
        name,
        status,
        items,
    })
}

pub async fn create_order(order: CreateOrder) -> String {
    let new_order = order.clone();
    let repo = order_repository::Repo::new().await;

    if let Ok(id) = repo.insert(new_order).await {
        // println!("created order: {:?}", id);
        repo.insert_order_items(order.items, id)
            .await
            .expect("error create order items");

        "user create success".to_string()
    } else {
        "error create order".to_string()
    }
}
