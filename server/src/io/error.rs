use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Bad Request: {0}")]
    BadRequest(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error; please try again later."),
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        eprintln!("{:?}", err);
        Error::InternalServerError
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(err: bcrypt::BcryptError) -> Self {
        eprintln!("{:?}", err);
        Error::InternalServerError
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        eprintln!("{:?}", err);
        Error::InternalServerError
    }
}

impl From<actix_http::Error> for Error {
    fn from(err: actix_http::Error) -> Self {
        eprintln!("{:?}", err);
        Error::InternalServerError
    }
}
