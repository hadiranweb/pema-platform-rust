use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Vendor {
    pub id: Uuid,
    pub name: String,
    pub contact_person: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

