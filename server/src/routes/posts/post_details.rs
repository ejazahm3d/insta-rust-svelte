use crate::{io::error::AppError, repos::PostsRepository};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostDetailsResponse {
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

pub async fn post_details(
    State(conn): State<PgPool>,
    Path(path): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let post_repository = PostsRepository { connection: &conn };
    let post = post_repository.find_one(&path).await?;

    dbg!(&post);
    match post {
        Some(p) => Ok(Json(p)),
        None => Err(AppError::NotFound),
    }
}
