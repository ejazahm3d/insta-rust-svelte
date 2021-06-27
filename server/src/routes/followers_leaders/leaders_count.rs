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
    let leader_count = &path;

    let people_you_follow_count = followers_repository.leaders_count(leader_count).await?;

    Ok(HttpResponse::Ok().json(people_you_follow_count))
}
