use crate::{
    repos::UserRepository,
    services::{Claims, Password, Token},
};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::PgPool;

use super::dtos::{LoginRequest, LoginResponse};

pub async fn login(
    body: web::Json<LoginRequest>,
    conn: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, HttpResponse> {
    let user_repository = UserRepository { connection: &conn };

    let user = user_repository
        .find_user_with_email(body.email.clone())
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch user {}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if user.is_none() {
        return Ok(HttpResponse::BadRequest().body("Invalid email or password"));
    }

    let user = user.unwrap();

    let is_password_match =
        Password::verify_password(body.password.as_str(), user.password.as_str()).map_err(|e| {
            eprintln!("Failed to verify user {}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if !is_password_match {
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

    Ok(HttpResponse::Ok().json(LoginResponse {
        id: user.id,
        email: user.email,
        username: user.username,
    }))
}
