use crate::{extractors::AuthUser, io::error::AppError, repos::PostsRepository};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn profile_posts(
    _auth_service: AuthUser,
    State(conn): State<PgPool>,
    Path(path): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let posts_repository = PostsRepository { connection: &conn };
    let user_id = &path;

    let posts = posts_repository.find_many_by_user_id(user_id).await?;

    Ok(Json(posts))
}
