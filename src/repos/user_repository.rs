use crate::models::User;
use sqlx::{Error, PgPool};

pub struct UserRepository<'a> {
    pub connection: &'a PgPool,
}

impl UserRepository<'_> {
    pub async fn find_user_with_email(&self, email: String) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
        User,
        "SELECT id, email, username, created_at, updated_at, avatar, password FROM users WHERE email = $1;",
        &email
    )
            .fetch_optional(self.connection)
            .await;

        return user;
    }
    pub async fn find_user_with_username(&self, username: String) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
        User,
        "SELECT id, email, username, created_at, updated_at, avatar, password FROM users WHERE username = $1;",
        &username
    )
            .fetch_optional(self.connection)
            .await;

        return user;
    }
}
