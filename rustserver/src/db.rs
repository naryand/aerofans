use crate::{
    model::{CreatePost, CreateReply, Post, Reply, UpdatePost, UpdateReply},
    StdErr,
};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug, Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn connect() -> Result<Self, StdErr> {
        let db_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(100)
            .min_connections(10)
            .connect(&db_url)
            .await?;
        Ok(Db { pool })
    }

    pub async fn create_post(&self, create_post: CreatePost) -> Result<Post, StdErr> {
        let post = sqlx::query_as("INSERT INTO posts (text) VALUES ($1) RETURNING *")
            .bind(&create_post.text)
            .fetch_one(&self.pool)
            .await?;
        Ok(post)
    }

    pub async fn read_all_posts(&self) -> Result<Vec<Post>, StdErr> {
        let posts = sqlx::query_as("SELECT * FROM posts")
            .fetch_all(&self.pool)
            .await?;
        Ok(posts)
    }

    pub async fn read_post(&self, post_id: i64) -> Result<Post, StdErr> {
        let post = sqlx::query_as("SELECT * FROM posts WHERE id = $1")
            .bind(post_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(post)
    }

    pub async fn update_post(&self, post_id: i64, update_post: UpdatePost) -> Result<Post, StdErr> {
        let post = sqlx::query_as("UPDATE posts SET text = $1 WHERE id = $2 RETURNING *")
            .bind(&update_post.text)
            .bind(post_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(post)
    }

    pub async fn delete_post(&self, post_id: i64) -> Result<(), StdErr> {
        sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(post_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn create_reply(
        &self,
        create_reply: CreateReply,
        post_id: i64,
    ) -> Result<Reply, StdErr> {
        let reply =
            sqlx::query_as("INSERT INTO replies (text, post_id) VALUES ($1, $2) RETURNING *")
                .bind(&create_reply.text)
                .bind(post_id)
                .fetch_one(&self.pool)
                .await?;
        Ok(reply)
    }

    pub async fn read_all_replies(&self, post_id: i64) -> Result<Vec<Reply>, StdErr> {
        let replies = sqlx::query_as("SELECT * FROM replies WHERE post_id = $1")
            .bind(post_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(replies)
    }

    pub async fn read_reply(&self, reply_id: i64, post_id: i64) -> Result<Reply, StdErr> {
        let reply = sqlx::query_as("SELECT * FROM replies WHERE id = $1 AND post_id = $2")
            .bind(reply_id)
            .bind(post_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(reply)
    }

    pub async fn update_reply(
        &self,
        reply_id: i64,
        post_id: i64,
        update_reply: UpdateReply,
    ) -> Result<Reply, StdErr> {
        let reply = sqlx::query_as(
            "UPDATE replies SET text = $1 WHERE id = $2 AND post_id = $3 RETURNING *",
        )
        .bind(&update_reply.text)
        .bind(reply_id)
        .bind(post_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(reply)
    }

    pub async fn delete_reply(&self, reply_id: i64, post_id: i64) -> Result<(), StdErr> {
        sqlx::query("DELETE FROM replies WHERE id = $1 AND post_id = $2")
            .bind(reply_id)
            .bind(post_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
