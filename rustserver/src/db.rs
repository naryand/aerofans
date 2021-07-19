use crate::{
    model::{
        CreatePost, CreateReply, LoginUser, Post, Reply, Token, UpdatePost, UpdateReply, User,
    },
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

    //
    // --------------------------- USERS ---------------------------
    //

    // password is hashed beforehand
    pub async fn create_user(&self, create_user: LoginUser) -> Result<(), StdErr> {
        sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
            .bind(&create_user.username)
            .bind(&create_user.password)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn read_user(&self, username: String) -> Result<User, StdErr> {
        let user = sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(&username)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn create_token(&self, create_token: Token) -> Result<(), StdErr> {
        sqlx::query("INSERT INTO tokens (id, user_id, expires_at) VALUES ($1, $2, $3)")
            .bind(&create_token.id)
            .bind(create_token.user_id)
            .bind(create_token.expires_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn read_token(&self, id: String) -> Result<Token, StdErr> {
        let token = sqlx::query_as("SELECT * FROM tokens WHERE id = $1")
            .bind(&id)
            .fetch_one(&self.pool)
            .await?;
        Ok(token)
    }

    //
    // --------------------------- POSTS ---------------------------
    //

    pub async fn create_post(&self, create_post: CreatePost, author: i64) -> Result<Post, StdErr> {
        let post = sqlx::query_as(
            "WITH updated AS (INSERT INTO posts (text, author) VALUES ($1, $2) RETURNING *) \
            SELECT updated.*, users.username FROM updated INNER JOIN users ON updated.author = users.id"
        )
            .bind(&create_post.text)
            .bind(author)
            .fetch_one(&self.pool)
            .await?;
        Ok(post)
    }

    pub async fn read_all_posts(&self) -> Result<Vec<Post>, StdErr> {
        let posts = sqlx::query_as(
            "SELECT posts.*, users.username FROM posts INNER JOIN users ON posts.author = users.id",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(posts)
    }

    pub async fn read_post(&self, post_id: i64) -> Result<Post, StdErr> {
        let post = sqlx::query_as(
            "SELECT posts.*, users.username \
            FROM posts INNER JOIN users ON posts.author = users.id \
            WHERE posts.id = $1",
        )
        .bind(post_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(post)
    }

    pub async fn update_post(&self, post_id: i64, update_post: UpdatePost) -> Result<Post, StdErr> {
        let post = sqlx::query_as(
            "WITH updated AS (UPDATE posts SET text = $1 WHERE id = $2 RETURNING *) \
            SELECT updated.*, users.username FROM updated INNER JOIN users ON updated.author = users.id \
            WHERE updated.id = $2"
        )
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

    //
    // --------------------------- REPLIES ---------------------------
    //

    pub async fn create_reply(
        &self,
        create_reply: CreateReply,
        post_id: i64,
        author: i64,
    ) -> Result<Reply, StdErr> {
        let reply =
            sqlx::query_as("WITH updated AS \
            (INSERT INTO replies (text, post_id, author) VALUES ($1, $2, $3) RETURNING *) \
            SELECT updated.*, users.username FROM updated INNER JOIN users ON updated.author = users.id"
        )
                .bind(&create_reply.text)
                .bind(post_id)
                .bind(author)
                .fetch_one(&self.pool)
                .await?;
        Ok(reply)
    }

    pub async fn read_all_replies(&self, post_id: i64) -> Result<Vec<Reply>, StdErr> {
        let replies = sqlx::query_as(
            "SELECT replies.*, users.username FROM replies \
        INNER JOIN users ON replies.author = users.id \
        WHERE replies.post_id = $1",
        )
        .bind(post_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(replies)
    }

    pub async fn read_reply(&self, reply_id: i64, post_id: i64) -> Result<Reply, StdErr> {
        let reply = sqlx::query_as(
            "SELECT replies.*, users.username FROM replies \
        INNER JOIN users ON replies.author = users.id \
        WHERE replies.id = $1 AND replies.post_id = $2",
        )
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
            "WITH updated AS (UPDATE replies SET text = $1 WHERE id = $2 AND post_id = $3 RETURNING *) \
            SELECT updated.*, users.username FROM updated INNER JOIN users ON updated.author = users.id \
            WHERE updated.id = $2 AND updated.post_id = $3",
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
