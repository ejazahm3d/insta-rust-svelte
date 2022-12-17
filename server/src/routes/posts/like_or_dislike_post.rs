use crate::{extractors::AuthUser, io::error::AppError, repos::PostsRepository};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn like_or_dislike_post(
    auth_service: AuthUser,
    State(conn): State<PgPool>,
    Path(path): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let post_repository = PostsRepository { connection: &conn };

    let post_id = &path;
    let user_id = auth_service.id;

    let post = post_repository.find_one(post_id).await?;

    if post.is_none() {
        return Err(AppError::NotFound);
    }

    let post_like = post_repository.find_one_like(post_id, &user_id).await?;

    match post_like {
        Some(_) => {
            post_repository.delete_like(post_id, &user_id).await?;

            Ok(())
        }
        None => {
            post_repository.insert_like(post_id, &user_id).await?;

            Ok(())
        }
    }
}
