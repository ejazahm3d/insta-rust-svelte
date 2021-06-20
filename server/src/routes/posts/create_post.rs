use crate::{extractors::AuthorizationService, io::error::Error, repos::PostsRepository};
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
    auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
) -> anyhow::Result<HttpResponse, Error> {
    let post_repository = PostsRepository {
        connection: conn.get_ref(),
    };

    let post = post_repository.insert_one(body.0, auth_service.id).await?;

    Ok(HttpResponse::Ok().json(post))
}
