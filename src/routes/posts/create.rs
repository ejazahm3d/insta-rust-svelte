use crate::{extractors::AuthorizationService, repos::PostsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize, Debug)]
pub struct CreatePostRequest {
    pub url: String,
    pub caption: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
}

pub async fn create_post(
    body: web::Json<CreatePostRequest>,
    _auth_service: AuthorizationService,
    _conn: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let post_repository = PostsRepository {
        connection: _conn.get_ref(),
    };

    println!("{:?}", body);

    let post = post_repository
        .insert_one(body.0, _auth_service.id)
        .await
        .map_err(|e| {
            eprintln!("Failed to insert post {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(post))
}
