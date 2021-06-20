use crate::{extractors::AuthorizationService, io::error::Error, repos::CommentsRepository};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct UpdateCommentRequest {
    pub contents: String,
}

pub async fn update_comment(
    auth_service: AuthorizationService,
    conn: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateCommentRequest>,
) -> anyhow::Result<HttpResponse, Error> {
    let comments_repository = CommentsRepository {
        connection: conn.get_ref(),
    };

    let comment_id = path.1;
    let user_id = auth_service.id;

    let comment = comments_repository.find_one(&comment_id).await?;

    if comment.is_none() {
        return Ok(HttpResponse::NotFound().body("Comment Not found"));
    }

    let comment = comment.unwrap();

    let is_owner = comment.user_id == user_id;

    match is_owner {
        true => {
            let contents = &body.contents;
            let comment = comments_repository
                .update_one(&comment_id, contents)
                .await?;

            Ok(HttpResponse::Ok().json(comment))
        }
        false => Ok(HttpResponse::Unauthorized().body("Not owner")),
    }
}
