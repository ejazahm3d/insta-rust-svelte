use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{
    models::{Comment, Like},
    routes::comments::CreateCommentRequest,
};

pub struct CommentsRepository<'a> {
    pub connection: &'a PgPool,
}

impl CommentsRepository<'_> {
    pub async fn find_many(&self, post_id: &Uuid) -> Result<Vec<Comment>, Error> {
        let comments = sqlx::query_as!(
            Comment,
            "SELECT * FROM comments WHERE post_id = $1;",
            post_id
        )
        .fetch_all(self.connection)
        .await;

        return comments;
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
        let comment = sqlx::query_as!(
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

        return comment;
    }

    pub async fn delete_one(&self, comment_id: &Uuid) -> Result<Comment, Error> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
        DELETE FROM comments WHERE id = $1 RETURNING *;
        "#,
            comment_id
        )
        .fetch_one(self.connection)
        .await;

        return comment;
    }

    pub async fn update_one(&self, comment_id: &Uuid, contents: &str) -> Result<Comment, Error> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
        UPDATE comments SET contents = $1 WHERE id = $2 RETURNING *;
        "#,
            contents,
            comment_id
        )
        .fetch_one(self.connection)
        .await;

        return comment;
    }

    pub async fn insert_like(&self, comment_id: &Uuid, user_id: &Uuid) -> Result<Like, Error> {
        let like = sqlx::query_as!(
            Like,
            r#"
        INSERT INTO likes (comment_id, user_id) 
        VALUES ($1, $2) RETURNING *;
        "#,
            comment_id,
            user_id
        )
        .fetch_one(self.connection)
        .await;

        like
    }

    pub async fn delete_like(&self, comment_id: &Uuid) -> Result<Like, Error> {
        let like = sqlx::query_as!(
            Like,
            r#"
        DELETE FROM likes WHERE comment_id = $1 RETURNING *;
        "#,
            comment_id
        )
        .fetch_one(self.connection)
        .await;

        like
    }

    pub async fn find_one_like(
        &self,
        comment_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<Option<Like>, Error> {
        let like = sqlx::query_as!(
            Like,
            "SELECT * FROM likes WHERE user_id = $1 AND comment_id = $2;",
            user_id,
            comment_id
        )
        .fetch_optional(self.connection)
        .await;

        like
    }
}
