use actix_web::{App, HttpServer};
use dotenvy::dotenv;

use crate::db::init_pg_pool;

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
            .service(users::scope())
            .service(categories::scope())
            .service(products::scope())
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
