use serde::{Deserialize, Serialize};

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

