use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Review {
    pub id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

