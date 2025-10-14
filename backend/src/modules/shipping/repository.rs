use sqlx::{PgPool, Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Shipping {
    pub id: Uuid,
    pub order_id: Uuid,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn create_shipping(pool: &PgPool, order_id: Uuid, address: String, city: String, state: String, zip_code: String, country: String) -> Result<Shipping, Error> {
    let shipping = sqlx::query_as::<_, Shipping>(
        "INSERT INTO shipping (id, order_id, address, city, state, zip_code, country, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(order_id)
    .bind(address)
    .bind(city)
    .bind(state)
    .bind(zip_code)
    .bind(country)
    .bind("pending")
    .fetch_one(pool)
    .await?;
    Ok(shipping)
}

pub async fn get_shipping_by_id(pool: &PgPool, shipping_id: Uuid) -> Result<Option<Shipping>, Error> {
    let shipping = sqlx::query_as::<_, Shipping>(
        "SELECT * FROM shipping WHERE id = $1"
    )
    .bind(shipping_id)
    .fetch_optional(pool)
    .await?;
    Ok(shipping)
}

pub async fn update_shipping_status(pool: &PgPool, shipping_id: Uuid, status: &str) -> Result<Option<Shipping>, Error> {
    let shipping = sqlx::query_as::<_, Shipping>(
        "UPDATE shipping SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(status)
    .bind(shipping_id)
    .fetch_optional(pool)
    .await?;
    Ok(shipping)
}

