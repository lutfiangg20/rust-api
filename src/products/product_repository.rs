use sqlx::PgPool;

use crate::{
    db::pool,
    products::product_model::{CreateProduct, Product},
};

pub struct Repo {
    db: PgPool,
}

impl Repo {
    pub async fn new() -> Self {
        Self { db: pool() }
    }

    pub async fn find_all(&self) -> Result<Vec<Product>, sqlx::Error> {
        let categories = sqlx::query_as::<_, Product>("select id,name,price from products")
            .fetch_all(&self.db)
            .await
            .expect("repo category error");
        Ok(categories)
    }

    pub async fn insert(&self, product: CreateProduct) -> Result<(), sqlx::Error> {
        sqlx::query("insert into products (name,price) values ($1,$2)")
            .bind(product.name)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
