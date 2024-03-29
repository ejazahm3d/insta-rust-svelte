use crate::{extractors::AuthorizationService, io::error::AppError, repos::CommentsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn like_or_dislike_comment(
    auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let comments_repository = CommentsRepository {
        connection: conn.get_ref(),
    };

    let comment_id = &path.1;

    let user_id = auth_service.id;

    let comment = comments_repository.find_one(comment_id).await?;

    if comment.is_none() {
        return Ok(HttpResponse::NotFound().body("Not found"));
    }

    let comment_like = comments_repository
        .find_one_like(comment_id, &user_id)
        .await?;

    match comment_like {
        Some(_) => {
            comments_repository.delete_like(comment_id).await?;

            Ok(HttpResponse::Ok().finish())
        }
        None => {
            comments_repository
                .insert_like(comment_id, &user_id)
                .await?;

            Ok(HttpResponse::Ok().finish())
        }
    }
}
