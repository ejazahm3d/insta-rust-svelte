use crate::{extractors::AuthUser, io::error::AppError, repos::PostsRepository};
use axum::{extract::State, response::IntoResponse, Json};
use sqlx::PgPool;

#[derive(serde::Deserialize, Debug)]
pub struct CreatePostRequest {
    pub url: String,
    pub caption: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
}

pub async fn create_post(
    State(conn): State<PgPool>,
    _auth_user: AuthUser,
    Json(body): Json<CreatePostRequest>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let post_repository = PostsRepository { connection: &conn };

    let post = post_repository.insert_one(body, _auth_user.id).await?;

    Ok((axum::http::StatusCode::CREATED, Json(post)))
}
