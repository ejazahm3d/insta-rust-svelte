use actix_web::{web, HttpResponse};

use crate::io::error::Error;
use crate::repos::FollowersRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn leaders_count(
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let followers_repository = FollowersRepository { connection: &conn };
    let follower_id = &path;

    let leaders_count = followers_repository.leaders_count(follower_id).await?;

    Ok(HttpResponse::Ok().json(leaders_count))
}
