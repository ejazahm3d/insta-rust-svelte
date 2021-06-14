use crate::repos::PostsRepository;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn list(conn: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let post_repository = PostsRepository { connection: &conn };
    let posts = post_repository.find_many().await.map_err(|e| {
        eprintln!("Failed to fetch posts {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(posts))
}
