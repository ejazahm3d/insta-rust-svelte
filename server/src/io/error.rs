use axum::{response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not Found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let response = match self {
            AppError::InternalServerError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!("Internal Server Error; please try again later.")),
            ),
            AppError::BadRequest(ref message) => {
                (axum::http::StatusCode::BAD_REQUEST, Json(json!(message)))
            }
            AppError::Unauthorized => (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!("Unauthorized!")),
            ),
            AppError::NotFound => (axum::http::StatusCode::NOT_FOUND, Json(json!("Not Found!"))),
        };

        response.into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        eprintln!("{:?}", err);
        AppError::InternalServerError
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        eprintln!("{:?}", err);
        AppError::InternalServerError
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        eprintln!("{:?}", err);
        AppError::InternalServerError
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        eprintln!("{:?}", err);
        AppError::InternalServerError
    }
}
