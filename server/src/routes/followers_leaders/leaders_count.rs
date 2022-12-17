use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use crate::io::error::AppError;
use crate::repos::FollowersRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn leaders_count(
    State(conn): State<PgPool>,
    Path(path): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let followers_repository = FollowersRepository { connection: &conn };
    let follower_id = &path;

    let leaders_count = followers_repository.leaders_count(follower_id).await?;

    Ok(Json(leaders_count))
}
