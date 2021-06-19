use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::models::Comment;

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

    // pub async fn insert_one(
    //     &self,
    //     post: CreatePostRequest,
    //     user_id: Uuid,
    // ) -> Result<Comment, Error> {
    //     let post = sqlx::query_as!(
    //         Comment,
    //         r#"
    //     INSERT INTO posts(url, caption, lat, lng, user_id) VALUES ($1, $2, $3, $4, $5) RETURNING *;
    //     "#,
    //         post.url,
    //         post.caption,
    //         post.lat,
    //         post.lng,
    //         user_id
    //     )
    //     .fetch_one(self.connection)
    //     .await;

    //     return post;
    // }
}
