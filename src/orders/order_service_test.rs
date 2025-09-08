#[cfg(test)]
mod db_test {
    use dotenvy::dotenv;

    use crate::{
        db::init_pg_pool,
        orders::{
            order_model::{CreateOrder, CreateOrderItem},
            order_service,
        },
    };

    #[tokio::test]
    async fn test_get_orders() {
        dotenv().ok();

        init_pg_pool().await;

        let orders = order_service::get_all_orders().await;

        let json = serde_json::to_string_pretty(&orders).unwrap();
        println!("orders: {}", json);
        assert!(orders.len() > 0);
    }

    #[tokio::test]
    async fn test_add_order() {
        dotenv().ok();
        init_pg_pool().await;

        let items = vec![
            CreateOrderItem {
                product_id: 1,
                price: 60000,
                quantity: 4,
            },
            CreateOrderItem {
                product_id: 3,
                price: 40000,
                quantity: 4,
            },
        ];

        let order = CreateOrder {
            user_id: 1,
            status: "pending".to_string(),
            items,
        };

        let service = order_service::create_order(order).await;
        assert_eq!(service, "order create success".to_string());
    }
}
