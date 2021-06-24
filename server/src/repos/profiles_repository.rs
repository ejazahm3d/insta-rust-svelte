use crate::routes::profiles::UserProfileResponse;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct ProfilesRepository<'a> {
    pub connection: &'a PgPool,
}

impl ProfilesRepository<'_> {
    pub async fn fine_one(&self, user_id: &Uuid) -> Result<Option<UserProfileResponse>, Error> {
        let user = sqlx::query_as!(
            UserProfileResponse,
            r#"
            SELECT id, email, username, created_at, updated_at, avatar 
            FROM users WHERE id = $1;
            "#,
            user_id
        )
        .fetch_optional(self.connection)
        .await;

        return user;
    }
}
