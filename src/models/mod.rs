use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub avatar: Option<String>,
    pub password: String,
}
