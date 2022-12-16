use crate::{io::error::AppError, repos::CommentsRepository};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsByPostResponse {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub contents: String,
    pub user_id: Uuid,
    pub username: String,
    pub post_id: Uuid,
    pub likes: Option<i64>,
}

pub async fn post_comments(
    conn: web::Data<PgPool>,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let comments_repository = CommentsRepository { connection: &conn };
    let comments = comments_repository.find_many(&post_id).await?;

    Ok(HttpResponse::Ok().json(comments))
}
