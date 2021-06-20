use crate::{extractors::AuthorizationService, io::error::Error, repos::CommentsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct CreateCommentRequest {
    pub contents: String,
}

pub async fn create_comment(
    body: web::Json<CreateCommentRequest>,
    auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> anyhow::Result<HttpResponse, Error> {
    let comments_repository = CommentsRepository {
        connection: conn.get_ref(),
    };

    let user_id = auth_service.id;
    let post_id = path;

    let comment = comments_repository
        .insert_one(&body.0, &user_id, &post_id)
        .await?;

    Ok(HttpResponse::Ok().json(comment))
}
