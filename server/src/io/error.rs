use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Bad Request: {0}")]
    BadRequest(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error; please try again later."),
            AppError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
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

impl From<actix_session::SessionInsertError> for AppError {
    fn from(err: actix_session::SessionInsertError) -> Self {
        eprintln!("{:?}", err);
        AppError::InternalServerError
    }
}

impl From<actix_session::SessionGetError> for AppError {
    fn from(err: actix_session::SessionGetError) -> Self {
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

impl From<actix_http::Error> for AppError {
    fn from(err: actix_http::Error) -> Self {
        eprintln!("{:?}", err);
        AppError::InternalServerError
    }
}
