use actix_http::error::ErrorInternalServerError;
use actix_session::UserSession;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use uuid::Uuid;

use crate::services::Token;

pub struct AuthorizationService {
    pub id: Uuid,
}

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let session = UserSession::get_session(_req);
        let token = session.get::<String>("jwt");
        if let Ok(_token) = token {
            match _token {
                Some(t) => match Token::verify(&t) {
                    Ok(_token) => ok(AuthorizationService {
                        id: _token.claims.sub,
                    }),
                    Err(_e) => err(ErrorUnauthorized("invalid token!")),
                },
                None => err(ErrorUnauthorized("blocked!")),
            }
        } else {
            err(ErrorInternalServerError("Something went wrong"))
        }
    }
}
