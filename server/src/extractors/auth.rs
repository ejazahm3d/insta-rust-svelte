use actix_session::SessionExt;
use actix_web::{
    dev,
    error::{ErrorInternalServerError, ErrorUnauthorized},
    Error, FromRequest, HttpRequest,
};
use futures::future::{err, ok, Ready};
use uuid::Uuid;

use crate::services::Token;

pub struct AuthorizationService {
    pub id: Uuid,
}

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let session = _req.get_session();
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
