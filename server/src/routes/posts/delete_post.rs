use crate::{extractors::AuthUser, io::error::AppError, repos::PostsRepository};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_post(
    auth_service: AuthUser,
    State(conn): State<PgPool>,
    Path(path): Path<Uuid>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let post_repository = PostsRepository { connection: &conn };

    let post_id = &path;
    let user_id = auth_service.id;

    let post = post_repository.find_one(post_id).await?;

    if post.is_none() {
        return Err(AppError::NotFound);
    }

    let post = post.unwrap();

    let is_owner = post.user_id == user_id;

    match is_owner {
        true => {
            // delete image
            let filepath = format!(".{}", post.url);
            tokio::task::spawn_blocking(|| std::fs::remove_file(filepath))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    AppError::InternalServerError
                })?
                .map_err(|e| {
                    eprintln!("{}", e);
                    AppError::InternalServerError
                })?;

            // delete post
            let post = post_repository.delete_one(post_id).await?;

            Ok(Json(post))
        }
        false => Err(AppError::Unauthorized),
    }
}
