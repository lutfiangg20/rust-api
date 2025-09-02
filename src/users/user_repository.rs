use sqlx::PgPool;

use crate::{
    db::pool,
    users::user_model::{CreateUser, User, UserWithPassword},
};

pub struct Repo {
    db: PgPool,
}

impl Repo {
    pub async fn new() -> Self {
        Self { db: pool() }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("select id,name,email,phone from users")
            .fetch_all(&self.db)
            .await
            .expect("repo user error");
        Ok(users)
    }

    pub async fn find_by_email(&self, email: String) -> Result<UserWithPassword, sqlx::Error> {
        let user = sqlx::query_as::<_, UserWithPassword>(
            r#"
        select id,name,password 
        from users 
        where email = $1
        "#,
        )
        .bind(email)
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    pub async fn insert(&self, user: CreateUser) -> Result<(), sqlx::Error> {
        sqlx::query("insert into users (name,email,password,phone) values ($1,$2,$3,$4)")
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(user.phone)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
