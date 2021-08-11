use crate::{extractors::AuthorizationService, io::error::Error, repos::PostsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn profile_posts(
    _auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let posts_repository = PostsRepository { connection: &conn };
    let user_id = &path;

    let posts = posts_repository.find_many_by_user_id(user_id).await?;

    Ok(HttpResponse::Ok().json(posts))
}
