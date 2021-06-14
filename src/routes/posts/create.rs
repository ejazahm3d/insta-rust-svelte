use crate::extractors::AuthorizationService;
use actix_web::HttpResponse;

pub async fn create_post(_auth_service: AuthorizationService) -> HttpResponse {
    HttpResponse::Ok().body(_auth_service.id)
}
