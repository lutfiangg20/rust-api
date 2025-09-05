use std::collections::HashMap;

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

    for order in order_repo.clone() {
        order_map.entry(order.id).or_default().push(order);
    }

    order_repo
        .into_iter()
        .map(move |order| {
            let items = order_map
                .get(&order.id)
                .cloned()
                .unwrap_or_default()
                .iter()
                .map(|item| OrderItem {
                    product_name: order.product_name.clone().unwrap_or_default(),
                    quantity: item.quantity.unwrap_or_default(),
                    price: item.price.unwrap_or_default(),
                })
                .collect();

            Order {
                id: order.id,
                name: order.name,
                status: "pending".to_string(),
                items,
            }
        })
        .collect()
}

pub async fn create_order(order: CreateOrder) -> String {
    let new_order = order.clone();
    let repo = order_repository::Repo::new().await;

    if let Ok(id) = repo.insert(new_order).await {
        println!("created order: {:?}", id);
        repo.insert_order_items(order.items, id)
            .await
            .expect("error create order items");

        "user create success".to_string()
    } else {
        "error create order".to_string()
    }
}
