use std::usize;

use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::io::error::AppError;
use crate::services::{Claims, Password, Token};

use crate::repos::UsersRepository;

#[derive(serde::Deserialize)]
pub struct SignUpRequest {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) avatar: Option<String>,
}

#[derive(serde::Serialize)]
pub struct SignUpResponse {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) username: String,
}

pub async fn sign_up(
    body: web::Json<SignUpRequest>,
    conn: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    let user_repository = UsersRepository {
        connection: conn.as_ref(),
    };

    let user_with_email = user_repository.find_user_with_email(&body.email).await?;

    if user_with_email.is_some() {
        return Ok(HttpResponse::BadRequest().body("User with email already exists"));
    }

    let user_with_username = user_repository
        .find_user_with_username(&body.username)
        .await?;

    if user_with_username.is_some() {
        return Ok(HttpResponse::BadRequest().body("User with username already exists"));
    }

    let hashed_password = Password::hash_password(&body.password)?;

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
        .await?;

    let expiry_date = Utc::now() + Duration::days(7);

    let token = Token::sign(Claims {
        sub: res.id,
        exp: expiry_date.timestamp() as usize,
    })?;

    // Save jwt in cookie session
    session.insert("jwt", token)?;

    Ok(HttpResponse::Created().json(SignUpResponse {
        id: res.id,
        email: res.email,
        username: res.username,
    }))
}
