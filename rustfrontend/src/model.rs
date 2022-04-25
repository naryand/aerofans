use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PostData {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub text: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostText {
    pub text: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CommentData {
    pub id: i64,
    pub post_id: i64,
    #[allow(dead_code)]
    user_id: i64,
    pub username: String,
    pub text: String,
    pub created_at: NaiveDateTime,
}

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
