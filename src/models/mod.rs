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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub url: String,
    pub caption: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub user_id: Uuid,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub contents: String,
    pub user_id: Uuid,
    pub post_id: Uuid,
}
