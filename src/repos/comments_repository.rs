use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{models::Comment, routes::posts::CreateCommentRequest};

pub struct CommentsRepository<'a> {
    pub connection: &'a PgPool,
}

impl CommentsRepository<'_> {
    pub async fn find_many(&self, post_id: &Uuid) -> Result<Vec<Comment>, Error> {
        let posts = sqlx::query_as!(
            Comment,
            "SELECT * FROM comments WHERE post_id = $1;",
            post_id
        )
        .fetch_all(self.connection)
        .await;

        return posts;
    }

    pub async fn find_one(&self, id: &Uuid) -> Result<Option<Comment>, Error> {
        let comment = sqlx::query_as!(Comment, "SELECT * FROM comments WHERE id = $1;", id)
            .fetch_optional(self.connection)
            .await;
        return comment;
    }

    pub async fn insert_one(
        &self,
        comment: &CreateCommentRequest,
        user_id: &Uuid,
        post_id: &Uuid,
    ) -> Result<Comment, Error> {
        let post = sqlx::query_as!(
            Comment,
            r#"
        INSERT INTO comments(contents, user_id, post_id) VALUES ($1, $2, $3) RETURNING *;
        "#,
            comment.contents,
            user_id,
            post_id
        )
        .fetch_one(self.connection)
        .await;

        return post;
    }

    pub async fn delete_one(&self, comment_id: &Uuid) -> Result<Comment, Error> {
        let post = sqlx::query_as!(
            Comment,
            r#"
        DELETE FROM comments WHERE id = $1 RETURNING *;
        "#,
            comment_id
        )
        .fetch_one(self.connection)
        .await;

        return post;
    }
}
