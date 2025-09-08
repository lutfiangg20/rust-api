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
        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT 
            p.id as id,
            p.name as name,
            p.price as price,
            c.name as category
            FROM products p LEFT JOIN categories c ON c.id = p.category_id
        "#,
        )
        .fetch_all(&self.db)
        .await
        .expect("repo category error");
        Ok(products)
    }

    pub async fn find_by_ids(&self, ids: Vec<i32>) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT 
            p.id as id,
            p.name as name,
            p.price as price,
            c.name as category
            FROM products p LEFT JOIN categories c ON c.id = p.category_id
            WHERE p.id = ANY($1)
        "#,
        )
        .bind(ids)
        .fetch_all(&self.db)
        .await?;
        Ok(products)
    }

    pub async fn insert(&self, product: CreateProduct) -> Result<(), sqlx::Error> {
        sqlx::query("insert into products (name,price,category_id) values ($1,$2,$3)")
            .bind(product.name)
            .bind(product.price)
            .bind(product.category_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
