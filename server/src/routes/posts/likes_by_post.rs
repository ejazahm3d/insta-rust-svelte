use crate::{io::error::AppError, repos::PostsRepository};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikesByPostResponse {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub user_id: Uuid,
    pub username: String,
    pub post_id: Option<Uuid>,
}

pub async fn likes_by_post(
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_repository = PostsRepository { connection: &conn };
    let post_id = &path;
    let likes = post_repository.find_many_likes(post_id).await?;

    Ok(HttpResponse::Ok().json(likes))
}
