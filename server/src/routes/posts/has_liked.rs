use crate::{extractors::AuthorizationService, io::error::Error, repos::PostsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn has_liked(
    auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let post_repository = PostsRepository {
        connection: conn.get_ref(),
    };

    let post_id = &path;
    let user_id = auth_service.id;

    let post = post_repository.find_one(post_id).await?;

    if post.is_none() {
        return Ok(HttpResponse::NotFound().body("Not found"));
    }

    let post_like = post_repository.find_one_like(post_id, &user_id).await?;

    match post_like {
        Some(_) => Ok(HttpResponse::Ok().json(true)),
        None => Ok(HttpResponse::Ok().json(false)),
    }
}
