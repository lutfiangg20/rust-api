use crate::users::{
    user_model::{CreateUser, User},
    user_repository,
};

pub async fn get_all_users() -> Vec<User> {
    user_repository::Repo::new()
        .await
        .find_all()
        .await
        .expect("allaala")
}

pub async fn create_user(user: CreateUser) -> String {
    user_repository::Repo::new()
        .await
        .insert(user)
        .await
        .expect("error create user");

    "user create success".to_string()
}
