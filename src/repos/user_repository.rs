use crate::{
    models::User,
    routes::auth::dtos::{SignUpRequest, SignUpResponse},
};
use sqlx::{Error, PgPool};

pub struct UserRepository<'a> {
    pub connection: &'a PgPool,
}

impl UserRepository<'_> {
    pub async fn find_user_with_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT id, email, username, created_at, updated_at, avatar, password 
        FROM users WHERE email = $1;"#,
            email
        )
        .fetch_optional(self.connection)
        .await;

        return user;
    }
    pub async fn find_user_with_username(&self, username: String) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT id, email, username, created_at, updated_at, avatar, password 
        FROM users WHERE username = $1;
        "#,
            &username
        )
        .fetch_optional(self.connection)
        .await;

        return user;
    }

    pub async fn find_user_with_id(&self, username: String) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, created_at, updated_at, avatar, password 
            FROM users WHERE username = $1;
            "#,
            &username
        )
        .fetch_optional(self.connection)
        .await;

        return user;
    }

    pub async fn insert_one(
        &self,
        user: SignUpRequest,
        hashed_password: &str,
    ) -> Result<SignUpResponse, Error> {
        let res = sqlx::query_as!(
            SignUpResponse,
            r#"
            INSERT into users(email, username, password, avatar) 
            VALUES($1, $2, $3, $4) RETURNING id, username, email;
            "#,
            user.email,
            user.username,
            hashed_password,
            user.avatar
        )
        .fetch_one(self.connection)
        .await;
        return res;
    }
}
