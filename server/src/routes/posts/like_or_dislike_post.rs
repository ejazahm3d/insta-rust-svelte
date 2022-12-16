use crate::{extractors::AuthorizationService, io::error::AppError, repos::PostsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn like_or_dislike_post(
    auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
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
        Some(_) => {
            post_repository.delete_like(post_id, &user_id).await?;

            Ok(HttpResponse::Ok().finish())
        }
        None => {
            post_repository.insert_like(post_id, &user_id).await?;

            Ok(HttpResponse::Ok().finish())
        }
    }
}
