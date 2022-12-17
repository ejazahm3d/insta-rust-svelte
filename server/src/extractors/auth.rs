use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_sessions::extractors::ReadableSession;
use uuid::Uuid;

use crate::{io::error::AppError, services::Token};

pub struct AuthUser {
    pub id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let session = parts
            .extract::<ReadableSession>()
            .await
            .map_err(|err| err.into_response())?;

        let token = session.get::<String>("jwt");
        if let Some(_token) = token {
            match Token::verify(&_token) {
                Ok(_token) => Ok(AuthUser {
                    id: _token.claims.sub,
                }),
                Err(_e) => Err(AppError::Unauthorized.into_response()),
            }
        } else {
            Err(AppError::Unauthorized.into_response())
        }
    }
}
