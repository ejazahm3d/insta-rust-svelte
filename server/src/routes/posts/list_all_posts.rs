use crate::{io::error::AppError, repos::PostsRepository};
use axum::{extract::State, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPostsResponse {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub url: String,
    pub caption: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub user_id: Uuid,
    pub username: String,
    pub likes: Option<i64>,
    pub comments: Option<i64>,
}

pub async fn list_all_posts(State(conn): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let post_repository = PostsRepository { connection: &conn };
    let posts = post_repository.find_many().await?;

    Ok(Json(posts))
}
