use actix_web::{
    Error, HttpResponse,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode, errors,
};

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

pub async fn guard(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    if let Some(Ok(auth_str)) = req.headers().get("Authorization").map(|h| h.to_str())
        && let Some(token) = auth_str.split_whitespace().nth(1)
    {
        match decode_token(token) {
            Ok(_) => {
                return next.call(req).await;
            }
            Err(_) => {
                let (req, _pl) = req.into_parts();
                let res = HttpResponse::Unauthorized().finish();
                return Ok(ServiceResponse::new(req, res));
            }
        }
    }
    let (req, _pl) = req.into_parts();
    let res = HttpResponse::Unauthorized().finish();
    Ok(ServiceResponse::new(req, res))
}

fn decode_token(jwt: &str) -> Result<TokenData<Claims>, errors::Error> {
    decode::<Claims>(
        jwt,
        &DecodingKey::from_secret("MY_SECRET".as_ref()),
        &Validation::default(),
    )
}
