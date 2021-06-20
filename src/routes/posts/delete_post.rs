use crate::{extractors::AuthorizationService, io::error::Error, repos::PostsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_post(
    _auth_service: AuthorizationService,
    _conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> anyhow::Result<HttpResponse, Error> {
    let post_repository = PostsRepository {
        connection: _conn.get_ref(),
    };

    let post = post_repository.delete_one(&path).await?;

    Ok(HttpResponse::Ok().json(post))
}
