use crate::{extractors::AuthUser, io::error::AppError, repos::CommentsRepository};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_comment(
    auth_service: AuthUser,
    State(conn): State<PgPool>,
    Path(path): Path<(Uuid, Uuid)>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let comments_repository = CommentsRepository { connection: &conn };

    let comment_id = path.1;
    let user_id = auth_service.id;

    let comment = comments_repository.find_one(&comment_id).await?;

    if comment.is_none() {
        return Err(AppError::NotFound);
    }

    let comment = comment.unwrap();

    let is_owner = comment.user_id == user_id;

    match is_owner {
        true => {
            let comment = comments_repository.delete_one(&comment_id).await?;

            Ok(Json(comment))
        }
        false => Err(AppError::Unauthorized),
    }
}
