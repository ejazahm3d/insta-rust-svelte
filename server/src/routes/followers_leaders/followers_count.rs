use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use crate::io::error::AppError;
use crate::repos::FollowersRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn followers_count(
    State(conn): State<PgPool>,
    Path(path): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let followers_repository = FollowersRepository { connection: &conn };
    let leader_id = &path;

    let followers_count = followers_repository.followers_count(leader_id).await?;

    Ok(Json(followers_count))
}
