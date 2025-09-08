#[cfg(test)]
mod db_test {
    use dotenvy::dotenv;

    use crate::{
        db::init_pg_pool,
        products::{product_model::CreateProduct, product_service},
    };

    #[tokio::test]
    async fn test_get_products() {
        dotenv().ok();
        init_pg_pool().await;

        let products = product_service::get_all_products().await;

        let json = serde_json::to_string_pretty(&products).unwrap();
        println!("products: {}", json);
        // assert!(users.len() > 0);
    }

    #[tokio::test]
    async fn test_add_products() {
        dotenv().ok();

        init_pg_pool().await;
        let product1 = CreateProduct {
            name: "baju sekolah".to_string(),
            price: 60000,
            category_id: 1,
        };

        let product2 = CreateProduct {
            name: "kipas angin".to_string(),
            price: 00000,
            category_id: 2,
        };
        let product3 = CreateProduct {
            name: "burger".to_string(),
            price: 40000,
            category_id: 3,
        };

        let insert = product_service::create_product(product1).await;
        let insert2 = product_service::create_product(product2).await;
        let insert3 = product_service::create_product(product3).await;
        assert_eq!(insert, "product create success".to_string());
        assert_eq!(insert2, "product create success".to_string());
        assert_eq!(insert3, "product create success".to_string());
    }
}
