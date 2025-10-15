use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreatePage {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub is_published: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdatePage {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>,
}

