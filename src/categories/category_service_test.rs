#[cfg(test)]
mod db_test {
    use dotenvy::dotenv;

    use crate::{
        categories::{category_model::CreateCategory, category_service},
        db::init_pg_pool,
    };

    #[tokio::test]
    async fn test_get_categories() {
        dotenv().ok();
        init_pg_pool().await;

        let categories = category_service::get_all_categories().await;

        let json = serde_json::to_string_pretty(&categories).unwrap();
        println!("categories: {}", json);
        // assert!(users.len() > 0);
    }

    #[tokio::test]
    async fn test_add_categories() {
        dotenv().ok();

        init_pg_pool().await;
        let category1 = CreateCategory {
            name: "fashion".to_string(),
        };
        let category2 = CreateCategory {
            name: "electronic".to_string(),
        };
        let category3 = CreateCategory {
            name: "food".to_string(),
        };

        let insert = category_service::create_category(category1).await;
        let insert2 = category_service::create_category(category2).await;
        let insert3 = category_service::create_category(category3).await;
        assert_eq!(insert, "category create success".to_string());
        assert_eq!(insert2, "category create success".to_string());
        assert_eq!(insert3, "category create success".to_string());
    }
}
