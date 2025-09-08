use sqlx::PgPool;

use crate::{
    db::pool,
    orders::order_model::{CreateOrder, CreateOrderItem, OrderQuery},
};

pub struct Repo {
    db: PgPool,
}

impl Repo {
    pub async fn new() -> Self {
        Self { db: pool() }
    }

    pub async fn find_all(&self) -> Result<Vec<OrderQuery>, sqlx::Error> {
        let orders = sqlx::query_as::<_, OrderQuery>(
            r#"
            SELECT 
            o.id as id,
            u.name as name,
            i.id as item_id,
            p.id as product_id,
            p.name as product_name,
            o.status as status,
            i.quantity as quantity,
            i.price as price
            FROM orders o
            LEFT JOIN users u
            on u.id = o.user_id
            LEFT JOIN order_items i
            on o.id = i.order_id
            LEFT JOIN products p
            on p.id = i.product_id
        "#,
        )
        .fetch_all(&self.db)
        .await
        .expect("repo orders error");
        Ok(orders)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Vec<OrderQuery>, sqlx::Error> {
        let orders = sqlx::query_as::<_, OrderQuery>(
            r#"
            SELECT 
            o.id as id,
            u.name as name,
            i.id as item_id,
            p.id as product_id,
            p.name as product_name,
            o.status as status,
            i.quantity as quantity,
            i.price as price
            FROM orders o
            LEFT JOIN users u
            on u.id = o.user_id
            LEFT JOIN order_items i
            on o.id = i.order_id
            LEFT JOIN products p
            on p.id = i.product_id
            where o.id = $1
        "#,
        )
        .bind(id)
        .fetch_all(&self.db)
        .await?;
        Ok(orders)
    }

    pub async fn insert(&self, order: CreateOrder) -> Result<i32, sqlx::Error> {
        let id: i32 =
            sqlx::query_scalar("insert into orders (user_id,status) values ($1,$2) returning id")
                .bind(order.user_id)
                .bind(order.status)
                .fetch_one(&self.db)
                .await?;

        Ok(id)
    }

    pub async fn insert_order_items(
        &self,
        orders: Vec<CreateOrderItem>,
        id: i32,
    ) -> Result<(), sqlx::Error> {
        let tx = self.db.begin().await?;

        for order in orders {
            sqlx::query(
                "insert into order_items (order_id,product_id,quantity,price) values ($1,$2,$3,$4)",
            )
            .bind(id)
            .bind(order.product_id)
            .bind(order.quantity)
            .bind(order.price)
            .execute(&self.db)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
