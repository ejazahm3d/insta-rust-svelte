use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::routes::posts::LikesByPostResponse;
use crate::{
    models::{Like, Post},
    routes::posts::{CreatePostRequest, ListPostsResponse},
};

pub struct PostsRepository<'a> {
    pub connection: &'a PgPool,
}

impl PostsRepository<'_> {
    pub async fn find_many(&self) -> Result<Vec<ListPostsResponse>, Error> {
        let posts = sqlx::query_as!(
            ListPostsResponse,
            r#"
    SELECT p.id,
            caption,
            url,
            lat,
            lng,
            p.updated_at,
            p.created_at,
            username,
            p.user_id,
            COUNT(distinct l.id) as likes,
            COUNT(distinct c.id) as comments
     
     FROM posts p
              JOIN users u
                   ON p.user_id = u.id
              LEFT JOIN likes l
                        ON p.id = l.post_id
              LEFT JOIN comments c
                        ON p.id = c.post_id
     GROUP BY p.id, u.id;
 "#
        )
        .fetch_all(self.connection)
        .await;

        posts
    }

    pub async fn find_one(&self, id: &Uuid) -> Result<Option<Post>, Error> {
        let post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1;", id)
            .fetch_optional(self.connection)
            .await;
        post
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

        post
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

        post
    }
    pub async fn insert_like(&self, post_id: &Uuid, user_id: &Uuid) -> Result<Like, Error> {
        let like = sqlx::query_as!(
            Like,
            r#"
        INSERT INTO likes (post_id, user_id) 
        VALUES ($1, $2) RETURNING *;
        "#,
            post_id,
            user_id
        )
        .fetch_one(self.connection)
        .await;

        like
    }

    pub async fn delete_like(&self, post_id: &Uuid) -> Result<Like, Error> {
        let like = sqlx::query_as!(
            Like,
            r#"
        DELETE FROM likes WHERE post_id = $1 RETURNING *;
        "#,
            post_id
        )
        .fetch_one(self.connection)
        .await;

        like
    }

    pub async fn find_one_like(
        &self,
        post_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<Option<Like>, Error> {
        let like = sqlx::query_as!(
            Like,
            "SELECT * FROM likes WHERE user_id = $1 AND post_id = $2;",
            user_id,
            post_id
        )
        .fetch_optional(self.connection)
        .await;

        like
    }

    pub async fn find_many_likes(&self, post_id: &Uuid) -> Result<Vec<LikesByPostResponse>, Error> {
        let likes = sqlx::query_as!(
            LikesByPostResponse,
            r#"
        SELECT l.id, l.created_at, username, user_id, post_id
        FROM likes l
        JOIN users u
            ON u.id = l.user_id
        WHERE l.post_id = $1;
        "#,
            post_id
        )
        .fetch_all(self.connection)
        .await;

        likes
    }
}
