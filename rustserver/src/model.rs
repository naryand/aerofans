use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Post struct
// contains all info for a Post
#[derive(Serialize, Debug, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i64,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

// Used for creating Posts
// Minimal subset of attributes to create a Post
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
    pub text: String,
}

// Used for editing Postss
// Editable attributes of a Post
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePost {
    pub text: String,
}
