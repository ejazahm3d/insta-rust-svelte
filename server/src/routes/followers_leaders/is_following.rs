use actix_web::{web, HttpResponse};

use crate::repos::FollowersRepository;
use crate::{extractors::AuthorizationService, io::error::Error};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn is_following(
    auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let followers_repository = FollowersRepository { connection: &conn };
    let follower_id = &auth_service.id;
    let leader_id = &path.to_owned();
    let follower = followers_repository
        .find_one(leader_id, follower_id)
        .await?;

    match follower {
        Some(_) => Ok(HttpResponse::Ok().json(true)),
        None => Ok(HttpResponse::Ok().json(false)),
    }
}
