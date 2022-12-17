use crate::{extractors::AuthUser, io::error::AppError, repos::ProfilesRepository};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

pub async fn profile_details(
    _auth_service: AuthUser,
    State(conn): State<PgPool>,
    Path(path): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let profiles_repository = ProfilesRepository { connection: &conn };
    let user_id = &path;

    let profile = profiles_repository.fine_one(user_id).await?;

    Ok(Json(profile))
}
