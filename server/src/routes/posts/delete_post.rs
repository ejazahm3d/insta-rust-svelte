use crate::{extractors::AuthorizationService, io::error::Error, repos::PostsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_post(
    auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> anyhow::Result<HttpResponse, Error> {
    let post_repository = PostsRepository {
        connection: conn.get_ref(),
    };

    let post_id = &path;
    let user_id = auth_service.id;

    let post = post_repository.find_one(post_id).await?;

    if post.is_none() {
        return Ok(HttpResponse::NotFound().body("Not found"));
    }

    let post = post.unwrap();

    let is_owner = post.user_id == user_id;

    match is_owner {
        true => {
            let post = post_repository.delete_one(post_id).await?;

            Ok(HttpResponse::Ok().json(post))
        }
        false => Ok(HttpResponse::Unauthorized().body("Not Authorized")),
    }
}
