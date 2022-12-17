use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use crate::repos::FollowersRepository;
use crate::{extractors::AuthUser, io::error::AppError};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn follow_or_unfollow(
    auth_service: AuthUser,
    State(conn): State<PgPool>,
    Path(path): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let followers_repository = FollowersRepository { connection: &conn };
    let follower_id = &auth_service.id;
    let leader_id = &path.to_owned();
    let follower = followers_repository
        .find_one(leader_id, follower_id)
        .await?;

    match follower {
        Some(_) => {
            let follower = followers_repository
                .unfollow(leader_id, follower_id)
                .await?;
            Ok(Json(follower))
        }
        None => {
            if follower_id == leader_id {
                return Err(AppError::BadRequest("cant follow your own self".into()));
            }

            let follower = followers_repository.follow(leader_id, follower_id).await?;
            Ok(Json(follower))
        }
    }
}
