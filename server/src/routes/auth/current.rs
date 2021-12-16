use actix_session::Session;
use actix_web::HttpResponse;

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

pub async fn current_user(session: Session) -> HttpResponse {
    if let Ok(token_string) = session.get::<String>("jwt") {
        match token_string {
            Some(t) => match Token::verify(&t) {
                Ok(_token) => HttpResponse::Ok().json(CurrentUserResponse {
                    user: Some(CurrentUser {
                        id: _token.claims.sub,
                    }),
                }),
                Err(_e) => HttpResponse::Ok().json(CurrentUserResponse { user: None }),
            },
            None => HttpResponse::Ok().json(CurrentUserResponse { user: None }),
        }
    } else {
        return HttpResponse::Ok().json(CurrentUserResponse { user: None });
    }
}
