use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::models::Follower;

pub struct FollowersRepository<'a> {
    pub connection: &'a PgPool,
}

impl FollowersRepository<'_> {
    pub async fn find_one(
        &self,
        leader_id: &Uuid,
        follower_id: &Uuid,
    ) -> Result<Option<Follower>, Error> {
        let follower = sqlx::query_as!(
            Follower,
            r#"
                SELECT * 
                FROM followers 
                WHERE leader_id = $1 AND follower_id = $2;
                "#,
            leader_id,
            follower_id
        )
        .fetch_optional(self.connection)
        .await;

        follower
    }
    pub async fn follow(&self, leader_id: &Uuid, follower_id: &Uuid) -> Result<Follower, Error> {
        let follower = sqlx::query_as!(
            Follower,
            r#"
            INSERT INTO followers(leader_id, follower_id)
            VALUES ($1, $2) RETURNING *;
            "#,
            leader_id,
            follower_id
        )
        .fetch_one(self.connection)
        .await;

        return follower;
    }

    pub async fn unfollow(&self, leader_id: &Uuid, follower_id: &Uuid) -> Result<Follower, Error> {
        let follower = sqlx::query_as!(
            Follower,
            r#"
            DELETE FROM followers WHERE leader_id = $1 AND follower_id = $2 RETURNING *;
            "#,
            leader_id,
            follower_id
        )
        .fetch_one(self.connection)
        .await;

        return follower;
    }

    pub async fn find_many_by_leader(&self, leader_id: &Uuid) -> Result<Vec<Follower>, Error> {
        let followers = sqlx::query_as!(
            Follower,
            r#"
            SELECT *
            FROM followers WHERE leader_id = $1;
            "#,
            leader_id
        )
        .fetch_all(self.connection)
        .await;

        return followers;
    }

    pub async fn find_many_by_follower(&self, follower_id: &Uuid) -> Result<Vec<Follower>, Error> {
        let leaders = sqlx::query_as!(
            Follower,
            r#"
            SELECT * 
            FROM followers WHERE follower_id = $1;
            "#,
            follower_id
        )
        .fetch_all(self.connection)
        .await;

        return leaders;
    }

    pub async fn followers_count(&self, leader_id: &Uuid) -> Result<Option<i64>, Error> {
        let count = sqlx::query!(
            r#"SELECT COUNT(id) as COUNT FROM followers where leader_id = $1"#,
            leader_id
        )
        .fetch_one(self.connection)
        .await?;

        return Ok(count.count);
    }

    pub async fn leaders_count(&self, follower_id: &Uuid) -> Result<Option<i64>, Error> {
        let count = sqlx::query!(
            r#"SELECT COUNT(id) as COUNT FROM followers where follower_id = $1"#,
            follower_id
        )
        .fetch_one(self.connection)
        .await?;

        return Ok(count.count);
    }
}
