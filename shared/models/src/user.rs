use serde::{Deserialize, Serialize};

use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}

