use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use crate::io::error::AppError;
use crate::repos::FollowersRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn followers(
    State(conn): State<PgPool>,
    Path(path): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let followers_repository = FollowersRepository { connection: &conn };
    let leader_id = &path;
    let people_following_you = followers_repository.find_many_by_leader(leader_id).await?;

    Ok(Json(people_following_you))
}
