use actix_web::{web, HttpResponse};

use crate::io::error::Error;
use crate::repos::FollowersRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn followers(
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let followers_repository = FollowersRepository { connection: &conn };
    let leader_id = &path;
    let people_following_you = followers_repository.find_many_by_leader(leader_id).await?;

    Ok(HttpResponse::Ok().json(people_following_you))
}
