use crate::{io::error::Error, repos::PostsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn list_all_posts(conn: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let post_repository = PostsRepository { connection: &conn };
    let posts = post_repository.find_many().await?;

    Ok(HttpResponse::Ok().json(posts))
}
