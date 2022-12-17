use crate::{extractors::AuthUser, io::error::AppError, repos::CommentsRepository};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct CreateCommentRequest {
    pub contents: String,
}

pub async fn create_comment(
    auth_service: AuthUser,
    State(conn): State<PgPool>,
    Path(path): Path<Uuid>,
    Json(body): Json<CreateCommentRequest>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let comments_repository = CommentsRepository { connection: &conn };

    let user_id = auth_service.id;
    let post_id = path;

    let comment = comments_repository
        .insert_one(&body, &user_id, &post_id)
        .await?;

    Ok(Json(comment))
}
