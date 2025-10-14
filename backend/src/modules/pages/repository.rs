use sqlx::{PgPool, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn create_page(pool: &PgPool, title: String, slug: String, content: String, is_published: bool) -> Result<Page, Error> {
    let page = sqlx::query_as::<_, Page>(
        "INSERT INTO pages (title, slug, content, is_published) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(title)
    .bind(slug)
    .bind(content)
    .bind(is_published)
    .fetch_one(pool)
    .await?;
    Ok(page)
}

pub async fn get_page_by_id(pool: &PgPool, page_id: i32) -> Result<Option<Page>, Error> {
    let page = sqlx::query_as::<_, Page>(
        "SELECT * FROM pages WHERE id = $1"
    )
    .bind(page_id)
    .fetch_optional(pool)
    .await?;
    Ok(page)
}

pub async fn update_page(pool: &PgPool, page_id: i32, title: Option<String>, slug: Option<String>, content: Option<String>, is_published: Option<bool>) -> Result<Option<Page>, Error> {
    let page = sqlx::query_as::<_, Page>(
        "UPDATE pages SET title = COALESCE($1, title), slug = COALESCE($2, slug), content = COALESCE($3, content), is_published = COALESCE($4, is_published), updated_at = NOW() WHERE id = $5 RETURNING *"
    )
    .bind(title)
    .bind(slug)
    .bind(content)
    .bind(is_published)
    .bind(page_id)
    .fetch_optional(pool)
    .await?;
    Ok(page)
}

pub async fn delete_page(pool: &PgPool, page_id: i32) -> Result<Option<Page>, Error> {
    let page = sqlx::query_as::<_, Page>(
        "DELETE FROM pages WHERE id = $1 RETURNING *"
    )
    .bind(page_id)
    .fetch_optional(pool)
    .await?;
    Ok(page)
}

pub async fn get_all_pages(pool: &PgPool) -> Result<Vec<Page>, Error> {
    let pages = sqlx::query_as::<_, Page>(
        "SELECT * FROM pages ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(pages)
}

pub async fn get_published_pages(pool: &PgPool) -> Result<Vec<Page>, Error> {
    let pages = sqlx::query_as::<_, Page>(
        "SELECT * FROM pages WHERE is_published = TRUE ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(pages)
}

