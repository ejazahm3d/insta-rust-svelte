use axum::{response::IntoResponse, Json};
use axum_sessions::extractors::ReadableSession;
use uuid::Uuid;

use crate::services::Token;

#[derive(serde::Serialize)]
pub struct CurrentUser {
    pub(crate) id: Uuid,
}

#[derive(serde::Serialize)]
pub struct CurrentUserResponse {
    pub(crate) user: Option<CurrentUser>,
}

pub async fn current_user(session: ReadableSession) -> impl IntoResponse {
    if let Some(token_string) = session.get::<String>("jwt") {
        match Token::verify(&token_string) {
            Ok(_token) => Json(CurrentUserResponse {
                user: Some(CurrentUser {
                    id: _token.claims.sub,
                }),
            }),
            Err(_e) => Json(CurrentUserResponse { user: None }),
        }
    } else {
        return Json(CurrentUserResponse { user: None });
    }
}
