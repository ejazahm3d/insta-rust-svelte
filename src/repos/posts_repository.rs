use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{models::Post, routes::posts::CreatePostRequest};

pub struct PostsRepository<'a> {
    pub connection: &'a PgPool,
}

impl PostsRepository<'_> {
    pub async fn find_many(&self) -> Result<Vec<Post>, Error> {
        let posts = sqlx::query_as!(Post, "SELECT * FROM posts;")
            .fetch_all(self.connection)
            .await;

        return posts;
    }

    pub async fn find_one(&self, id: &Uuid) -> Result<Option<Post>, Error> {
        let post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1;", id)
            .fetch_optional(self.connection)
            .await;
        return post;
    }

    pub async fn insert_one(&self, post: CreatePostRequest, user_id: Uuid) -> Result<Post, Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
        INSERT INTO posts(url, caption, lat, lng, user_id) VALUES ($1, $2, $3, $4, $5) RETURNING *;
        "#,
            post.url,
            post.caption,
            post.lat,
            post.lng,
            user_id
        )
        .fetch_one(self.connection)
        .await;

        return post;
    }

    pub async fn delete_one(&self, post_id: &Uuid) -> Result<Post, Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
        DELETE FROM posts WHERE id = $1 RETURNING *;
        "#,
            post_id
        )
        .fetch_one(self.connection)
        .await;

        return post;
    }
}
