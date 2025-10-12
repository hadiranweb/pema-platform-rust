use sqlx::{PgPool, Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Page {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn create_page(pool: &PgPool, title: String, slug: String, content: String, is_published: bool) -> Result<Page, Error> {
    let page = sqlx::query_as::<_, Page>(
        "INSERT INTO pages (id, title, slug, content, is_published) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(title)
    .bind(slug)
    .bind(content)
    .bind(is_published)
    .fetch_one(pool)
    .await?;
    Ok(page)
}

pub async fn get_page_by_id(pool: &PgPool, page_id: Uuid) -> Result<Option<Page>, Error> {
    let page = sqlx::query_as::<_, Page>(
        "SELECT * FROM pages WHERE id = $1"
    )
    .bind(page_id)
    .fetch_optional(pool)
    .await?;
    Ok(page)
}

pub async fn update_page(pool: &PgPool, page_id: Uuid, title: Option<String>, slug: Option<String>, content: Option<String>, is_published: Option<bool>) -> Result<Option<Page>, Error> {
    let mut query_parts = Vec::new();
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
    let mut param_idx = 1;

    if let Some(t) = title {
        query_parts.push(format!("title = ${}", param_idx));
        params.push(Box::new(t));
        param_idx += 1;
    }
    if let Some(s) = slug {
        query_parts.push(format!("slug = ${}", param_idx));
        params.push(Box::new(s));
        param_idx += 1;
    }
    if let Some(c) = content {
        query_parts.push(format!("content = ${}", param_idx));
        params.push(Box::new(c));
        param_idx += 1;
    }
    if let Some(p) = is_published {
        query_parts.push(format!("is_published = ${}", param_idx));
        params.push(Box::new(p));
        param_idx += 1;
    }

    if query_parts.is_empty() {
        return get_page_by_id(pool, page_id).await;
    }

    let query_str = format!("UPDATE pages SET {} WHERE id = ${} RETURNING *", query_parts.join(", "), param_idx);
    params.push(Box::new(page_id));

    let mut query = sqlx::query_as::<_, Page>(&query_str);
    for param in params {
        query = query.bind(param);
    }

    let page = query.fetch_optional(pool).await?;
    Ok(page)
}

pub async fn delete_page(pool: &PgPool, page_id: Uuid) -> Result<Option<Page>, Error> {
    let page = sqlx::query_as::<_, Page>(
        "DELETE FROM pages WHERE id = $1 RETURNING *"
    )
    .bind(page_id)
    .fetch_optional(pool)
    .await?;
    Ok(page)
}

