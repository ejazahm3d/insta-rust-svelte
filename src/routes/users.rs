use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::{types::Uuid, PgPool};

#[derive(Debug, serde::Serialize)]
struct User {
    id: Uuid,
    email: String,
    username: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    avatar: Option<String>,
}

pub async fn get_all_users(conn: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let res = sqlx::query_as!(
        User,
        "SELECT id, email, username, created_at, updated_at, avatar FROM users;"
    )
    .fetch_all(conn.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(res))
}
