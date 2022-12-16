use actix_web::{web, HttpResponse};

use crate::repos::FollowersRepository;
use crate::{extractors::AuthorizationService, io::error::AppError};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn follow_or_unfollow(
    auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
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
            Ok(HttpResponse::Ok().json(follower))
        }
        None => {
            if follower_id == leader_id {
                return Ok(HttpResponse::BadRequest().body("cant follow your own self"));
            }

            let follower = followers_repository.follow(leader_id, follower_id).await?;
            Ok(HttpResponse::Ok().json(follower))
        }
    }
}
