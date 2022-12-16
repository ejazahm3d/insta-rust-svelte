use actix_web::{web, HttpResponse};

use crate::io::error::AppError;
use crate::repos::FollowersRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn followers_count(
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let followers_repository = FollowersRepository { connection: &conn };
    let leader_id = &path;

    let followers_count = followers_repository.followers_count(leader_id).await?;

    Ok(HttpResponse::Ok().json(followers_count))
}
