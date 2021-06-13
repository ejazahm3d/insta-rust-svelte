use actix_session::Session;
use actix_web::HttpResponse;

pub async fn logout(session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::Ok().finish()
}
