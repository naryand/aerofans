use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PostData {
    pub id: i64,
    author: i64,
    pub username: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommentData {
    id: i64,
    post_id: i64,
    author: i64,
    pub username: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
}
