use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::models::User;

pub async fn get_all_users(conn: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let res = sqlx::query_as!(
        User,
        "SELECT id, email, username, created_at, updated_at, avatar, password FROM users;"
    )
    .fetch_all(conn.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(res))
}
