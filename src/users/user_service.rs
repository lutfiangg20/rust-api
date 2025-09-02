use bcrypt::hash;

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
    let password_hash = tokio::task::spawn_blocking(move || hash(user.password, 5).unwrap())
        .await
        .unwrap();

    let new_user = CreateUser {
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        password: password_hash,
        phone: user.phone.to_owned(),
    };

    user_repository::Repo::new()
        .await
        .insert(new_user)
        .await
        .expect("error create user");

    "user create success".to_string()
}
