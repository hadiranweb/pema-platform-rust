use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub domain: String,
    pub config: TenantConfig,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantConfig {
    pub database_url: Option<String>,
    pub features: Vec<String>,
    pub limits: TenantLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantLimits {
    pub max_users: u32,
    pub max_storage_mb: u32,
    pub max_api_calls_per_hour: u32,
}

pub struct TenantManager {
    tenants: Arc<RwLock<HashMap<String, Tenant>>>,
}

impl TenantManager {
    pub fn new() -> Self {
        Self {
            tenants: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_tenant(&self, domain: &str) -> Option<Tenant> {
        let tenants = self.tenants.read().await;
        tenants.get(domain).cloned()
    }

    pub async fn add_tenant(&self, tenant: Tenant) {
        let mut tenants = self.tenants.write().await;
        tenants.insert(tenant.domain.clone(), tenant);
    }

    pub async fn remove_tenant(&self, domain: &str) -> Option<Tenant> {
        let mut tenants = self.tenants.write().await;
        tenants.remove(domain)
    }

    pub async fn list_tenants(&self) -> Vec<Tenant> {
        let tenants = self.tenants.read().await;
        tenants.values().cloned().collect()
    }

    pub async fn update_tenant(&self, domain: &str, tenant: Tenant) -> bool {
        let mut tenants = self.tenants.write().await;
        if tenants.contains_key(domain) {
            tenants.insert(domain.to_string(), tenant);
            true
        } else {
            false
        }
    }
}

impl Default for TenantManager {
    fn default() -> Self {
        Self::new()
    }
}