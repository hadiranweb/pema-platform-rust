use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreatePageDto {
    pub title: String,
    pub slug: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdatePageDto {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
}

