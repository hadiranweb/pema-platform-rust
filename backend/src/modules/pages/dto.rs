use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePageDto {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub is_published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePageDto {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>,
}

