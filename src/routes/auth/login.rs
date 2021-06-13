use crate::{
    models::User,
    services::{Claims, Password, Token},
};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::PgPool;

use super::dtos::LoginRequest;

pub async fn login(
    body: web::Json<LoginRequest>,
    conn: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, HttpResponse> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, username, created_at, updated_at, avatar, password FROM users WHERE email = $1;",
        body.email
    )
    .fetch_optional(conn.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch user {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if user.is_none() {
        return Ok(HttpResponse::BadRequest().body("Invalid email or password"));
    }

    let user = user.unwrap();

    let is_match = Password::verify_password(body.password.as_str(), user.password.as_str())
        .map_err(|e| {
            eprintln!("Failed to verify user {}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if !is_match {
        return Ok(HttpResponse::BadRequest().body("Invalid email or password"));
    }

    let _date = Utc::now() + Duration::days(7);

    let token = Token::sign(Claims {
        sub: user.id.to_string(),
        exp: _date.timestamp() as usize,
    })
    .map_err(|e| {
        eprintln!("Failed to sign user {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    // Save jwt in cookie session
    session.insert("jwt", token)?;

    Ok(HttpResponse::Ok().finish())
}
