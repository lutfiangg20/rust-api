use std::{env, time::Duration};

use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::OnceCell;

static PG_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn init_pg_pool() -> PgPool {
    PG_POOL
        .get_or_init(|| async {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

            PgPoolOptions::new()
                .max_connections(20)
                .acquire_timeout(Duration::from_secs(10))
                .connect(database_url.as_str())
                .await
                .expect("Failed to connect to Postgres")
        })
        .await
        .clone()
}

pub fn pool() -> PgPool {
    PG_POOL.get().expect("Pool not initiated").clone()
}
