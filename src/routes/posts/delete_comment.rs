use crate::{extractors::AuthorizationService, io::error::Error, repos::CommentsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_comment(
    _auth_service: AuthorizationService,
    _conn: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> anyhow::Result<HttpResponse, Error> {
    let comments_repository = CommentsRepository {
        connection: _conn.get_ref(),
    };

    let comment_id = path.1;

    let comment = comments_repository.delete_one(&comment_id).await?;

    Ok(HttpResponse::Ok().json(comment))
}
