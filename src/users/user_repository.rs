use sqlx::PgPool;

use crate::{
    db::pool,
    users::user_model::{CreateUser, User},
};

pub struct Repo {
    db: PgPool,
}

impl Repo {
    pub async fn new() -> Self {
        Self { db: pool() }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("select id,name,email from users")
            .fetch_all(&self.db)
            .await
            .expect("repo user error");
        Ok(users)
    }

    pub async fn insert(&self, user: CreateUser) -> Result<(), sqlx::Error> {
        sqlx::query("insert into users (name,email,password) values ($1,$2,$3)")
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
