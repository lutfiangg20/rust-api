#[cfg(test)]
mod db_test {
    use dotenvy::dotenv;

    use crate::{
        db::init_pg_pool,
        users::{user_model::CreateUser, user_service},
    };

    #[tokio::test]
    async fn test_get_user() {
        dotenv().ok();
        init_pg_pool().await;

        let users = user_service::get_all_users().await;

        let json = serde_json::to_string_pretty(&users).unwrap();
        println!("orders: {}", json);
        // assert!(users.len() > 0);
    }

    #[tokio::test]
    async fn test_add_user() {
        dotenv().ok();

        init_pg_pool().await;
        let user = CreateUser {
            name: "test".to_string(),
            email: "test@test.com".to_string(),
            phone: "0213123".to_string(),
            password: "test".to_string(),
        };

        let insert = user_service::create_user(user).await;
        assert_eq!(insert, "user create success".to_string());
    }
}
