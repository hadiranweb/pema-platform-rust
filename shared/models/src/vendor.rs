use serde::{Deserialize, Serialize};

use uuid::Uuid;
use chrono::{DateTime, Utc};

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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateVendor {
    pub name: String,
    pub contact_person: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateVendor {
    pub name: Option<String>,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

