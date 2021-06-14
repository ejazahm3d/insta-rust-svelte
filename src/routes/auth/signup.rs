use std::usize;

use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::{
    routes::auth::dtos::SignUpResponse,
    services::{Claims, Password, Token},
};

use super::dtos::SignUpRequest;
use crate::repos::UserRepository;

pub async fn sign_up(
    body: web::Json<SignUpRequest>,
    conn: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, HttpResponse> {
    let user_repository = UserRepository {
        connection: conn.as_ref(),
    };

    let user_with_email = user_repository
        .find_user_with_email(&body.email)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch user {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if user_with_email.is_some() {
        return Ok(HttpResponse::BadRequest().body("User with email already exists"));
    }

    let user_with_username = user_repository
        .find_user_with_username(body.username.clone())
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch user {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if user_with_username.is_some() {
        return Ok(HttpResponse::BadRequest().body("User with username already exists"));
    }

    let hashed_password = Password::hash_password(&body.password).map_err(|e| {
        eprintln!("Failed to hash password {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let res = user_repository
        .insert_one(
            SignUpRequest {
                avatar: body.avatar.clone(),
                email: body.email.clone(),
                password: body.password.clone(),
                username: body.username.clone(),
            },
            &hashed_password,
        )
        .await
        .map_err(|e| {
            eprintln!("Failed to add user {}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let _date = Utc::now() + Duration::days(7);

    let token = Token::sign(Claims {
        sub: body.email.clone(),
        exp: _date.timestamp() as usize,
    })
    .map_err(|e| {
        eprintln!("Failed to sign user {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    // Save jwt in cookie session
    session.insert("jwt", token)?;

    Ok(HttpResponse::Created().json(SignUpResponse {
        id: res.id,
        email: res.email,
        username: res.username,
    }))
}
