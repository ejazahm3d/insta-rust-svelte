use crate::{io::error::AppError, repos::CommentsRepository};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikesByCommentResponse {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub user_id: Uuid,
    pub username: String,
    pub comment_id: Option<Uuid>,
}

pub async fn likes_by_comment(
    State(conn): State<PgPool>,
    Path(path): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let comments_repository = CommentsRepository { connection: &conn };
    let comment_id = &path.1;
    let likes = comments_repository.find_many_likes(comment_id).await?;

    Ok(Json(likes))
}
