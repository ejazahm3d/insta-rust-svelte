use crate::{io::error::Error, repos::CommentsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn post_comments(
    conn: web::Data<PgPool>,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let comments_repository = CommentsRepository { connection: &conn };
    let comments = comments_repository.find_many(&post_id).await?;

    Ok(HttpResponse::Ok().json(comments))
}
