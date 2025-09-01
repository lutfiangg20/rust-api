use std::{env, time::Duration};

use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_pg_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(10))
        .connect(database_url.as_str())
        .await?;
    Ok(pool)
}
