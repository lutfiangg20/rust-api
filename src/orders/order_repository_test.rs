#[cfg(test)]
mod db_test {
    use dotenvy::dotenv;

    use crate::{db::init_pg_pool, orders::order_repository};

    #[tokio::test]
    async fn test_get_orders() {
        dotenv().ok();

        init_pg_pool().await;
        let repo = order_repository::Repo::new().await;

        let orders = repo.find_all().await.expect("repo orders error");

        let json = serde_json::to_string_pretty(&orders).unwrap();
        println!("orders: {}", json);
        assert!(orders.len() > 0);
    }
}
