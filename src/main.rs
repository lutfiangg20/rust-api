use actix_web::{App, HttpServer, middleware::NormalizePath};
use dotenvy::dotenv;

use crate::db::init_pg_pool;

mod auth;
mod categories;
mod common;
mod db;
mod products;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    init_pg_pool().await;

    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::trim())
            .service(users::scope())
            .service(categories::scope())
            .service(products::scope())
            .service(auth::scope())
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
