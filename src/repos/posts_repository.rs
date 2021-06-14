use sqlx::{Error, PgPool};

use crate::models::Post;

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
}
