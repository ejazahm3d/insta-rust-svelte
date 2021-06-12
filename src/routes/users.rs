use actix_web::HttpResponse;

pub async fn get_all_users() -> HttpResponse {
    HttpResponse::Ok().finish()
}
