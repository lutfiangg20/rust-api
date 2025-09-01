use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod categories;
mod common;
mod db;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(users::scope())
            .service(categories::scope())
        // .service(user_controller::hello)
        // .service(user_controller::users)
        // .service(user_controller::create_user)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
