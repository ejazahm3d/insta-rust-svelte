use crate::{io::error::Error, repos::PostsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn all_likes_by_post(
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let post_repository = PostsRepository { connection: &conn };
    let post_id = &path;
    let likes = post_repository.find_many_likes(post_id).await?;

    Ok(HttpResponse::Ok().json(likes))
}
