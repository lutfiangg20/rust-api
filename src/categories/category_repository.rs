use sqlx::PgPool;

use crate::{
    categories::category_model::{Category, CreateCategory},
    db::pool,
};

pub struct Repo {
    db: PgPool,
}

impl Repo {
    pub async fn new() -> Self {
        Self { db: pool() }
    }

    pub async fn find_all(&self) -> Result<Vec<Category>, sqlx::Error> {
        let categories = sqlx::query_as::<_, Category>("select id,name from categories")
            .fetch_all(&self.db)
            .await
            .expect("repo category error");
        Ok(categories)
    }

    pub async fn insert(&self, category: CreateCategory) -> Result<(), sqlx::Error> {
        sqlx::query("insert into categories (name) values ($1)")
            .bind(category.name)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
