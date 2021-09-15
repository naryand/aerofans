use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct PostData {
    pub id: i64,
    author: i64,
    pub username: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostText {
    pub text: String,
}

// contains all info for a comment reply
#[derive(Deserialize, Debug, Clone)]
pub struct CommentData {
    id: i64,
    post_id: i64,
    author: i64,
    pub username: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

// Used for logging in or registering Users
// Minimal subset of attributes to create a User
#[derive(Debug, Clone, Serialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

// Login response
#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub status: bool,
    pub message: String,
}
