use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

//
// --------------------------- USERS ---------------------------
//

// User struct
// contains all info for a User
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

// Used for logging in or registering Users
// Minimal subset of attributes to create a User
#[derive(Debug, Clone, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

// Login response
#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub status: bool,
    pub message: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Token {
    pub id: String,
    pub user_id: i64,
    pub expires_at: DateTime<Utc>,
}

pub struct AuthUser {
    pub id: i64,
}

//
// --------------------------- POSTS ---------------------------
//

// Post struct
// contains all info for a Post
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Post {
    pub id: i64,
    pub author: i64,
    pub username: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

// Used for creating Posts
// Minimal subset of attributes to create a Post
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePost {
    pub text: String,
}

// Used for editing Posts
// Editable attributes of a Post
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePost {
    pub text: String,
}

//
// --------------------------- REPLIES ---------------------------
//

// Reply struct
// contains all info for a comment reply
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Reply {
    pub id: i64,
    pub post_id: i64,
    pub author: i64,
    pub username: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

// Used for creating Replies
// Minimal subset of attributes to create a reply
#[derive(Debug, Clone, Deserialize)]
pub struct CreateReply {
    pub text: String,
}

// Used for editing Postss
// Editable attributes of a Reply
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateReply {
    pub text: String,
}
