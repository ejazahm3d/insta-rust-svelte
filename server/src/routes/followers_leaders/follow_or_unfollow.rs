use actix_web::{web, HttpResponse};

use crate::repos::FollowersRepository;
use crate::{extractors::AuthorizationService, io::error::Error};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct FollowUnfollowRequest {
    pub leader_id: Uuid,
}

pub async fn follow_or_unfollow(
    auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    body: web::Json<FollowUnfollowRequest>,
) -> Result<HttpResponse, Error> {
    let followers_repository = FollowersRepository { connection: &conn };
    let follower_id = &auth_service.id;
    let leader_id = &body.leader_id;
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
            let follower = followers_repository.follow(leader_id, follower_id).await?;
            Ok(HttpResponse::Ok().json(follower))
        }
    }
}
