use crate::{extractors::AuthorizationService, io::error::Error, repos::ProfilesRepository};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub avatar: Option<String>,
}

pub async fn profile_details(
    _auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let profiles_repository = ProfilesRepository { connection: &conn };
    let user_id = &path;

    let profile = profiles_repository.fine_one(user_id).await?;

    Ok(HttpResponse::Ok().json(profile))
}
