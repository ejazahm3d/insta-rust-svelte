use std::{collections::HashMap, usize};

use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::{
    models::User,
    services::{Claims, Password, Token},
};

#[derive(serde::Deserialize)]
pub struct LoginDto {
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct SignUpDto {
    email: String,
    username: String,
    password: String,
    avatar: Option<String>,
}

pub async fn sign_up(
    body: web::Json<SignUpDto>,
    conn: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, HttpResponse> {
    let hashed_password = Password::hash_password(body.password.as_str()).map_err(|e| {
        eprintln!("Failed to hash password {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

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

    if user.is_some() {
        if user.unwrap().username.eq(&body.username) {
            return Ok(HttpResponse::BadRequest().body("User with email already exists"));
        } else {
            return Ok(HttpResponse::BadRequest().body("User with email already exists"));
        }
    }

    let res = sqlx::query!(
        "INSERT into users(email, username, password, avatar) VALUES($1, $2, $3, $4) RETURNING id, username;",
        body.email,
        body.username,
        hashed_password,
        body.avatar
    )
    .fetch_one(conn.as_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to add user {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let mut map: HashMap<&str, String> = HashMap::new();

    map.insert("id", res.id.to_string());
    map.insert("username", res.username);

    let _date = Utc::now() + Duration::days(7);

    let token = Token::sign(Claims {
        sub: body.email.clone(),
        exp: _date.timestamp() as usize,
    })
    .map_err(|e| {
        eprintln!("Failed to sign user {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    session.insert("jwt", token)?;

    Ok(HttpResponse::Created().json(map))
}

pub async fn login(
    body: web::Json<LoginDto>,
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
        sub: user.email,
        exp: _date.timestamp() as usize,
    })
    .map_err(|e| {
        eprintln!("Failed to sign user {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    session.insert("jwt", token)?;

    Ok(HttpResponse::Ok().body(format!("{} {}", body.email, body.password)))
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().finish()
}
