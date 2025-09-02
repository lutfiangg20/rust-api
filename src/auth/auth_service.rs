use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    auth::auth_model::{Claims, Login},
    users::user_repository,
};

pub async fn login(user_login: Login) -> bool {
    match user_repository::Repo::new()
        .await
        .find_by_email(user_login.email)
        .await
    {
        Ok(user) => match verify(&user_login.password, &user.password) {
            Ok(true) => true,
            Ok(false) => false,
            Err(_) => false,
        },
        Err(_) => false,
    }
}

pub async fn generate_token(email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("MY_SECRET".as_ref()),
    )
}
