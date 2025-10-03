use models::vendor::Vendor;
use models::pagination::PaginatedResponse;
use super::api::{ApiService, ApiResult};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub active: Option<bool>,
    pub verified: Option<bool>,
}

impl Default for VendorQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            search: None,
            active: None,
            verified: None,
        }
    }
}

impl VendorQuery {
    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(search) = &self.search {
            params.push(format!("search={}", urlencoding::encode(search)));
        }
        if let Some(active) = self.active {
            params.push(format!("active={}", active));
        }
        if let Some(verified) = self.verified {
            params.push(format!("verified={}", verified));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVendor {
    pub name: String,
    pub description: Option<String>,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVendor {
    pub name: Option<String>,
    pub description: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
    pub active: Option<bool>,
    pub verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorStats {
    pub total_vendors: u32,
    pub active_vendors: u32,
    pub verified_vendors: u32,
    pub new_vendors_today: u32,
    pub new_vendors_this_week: u32,
    pub new_vendors_this_month: u32,
    pub total_products: u32,
    pub total_sales: f64,
}

pub struct VendorService {
    api: ApiService,
}

impl VendorService {
    pub fn new(api: ApiService) -> Self {
        Self { api }
    }

    /// Get all vendors with optional filtering and pagination
    pub async fn get_vendors(&self, query: Option<VendorQuery>) -> ApiResult<PaginatedResponse<Vendor>> {
        let query_string = query.unwrap_or_default().to_query_string();
        let endpoint = format!("vendors{}", query_string);
        self.api.get(&endpoint).await
    }

    /// Get a single vendor by ID
    pub async fn get_vendor(&self, id: Uuid) -> ApiResult<Vendor> {
        let endpoint = format!("vendors/{}", id);
        self.api.get(&endpoint).await
    }

    /// Create a new vendor
    pub async fn create_vendor(&self, vendor: CreateVendor) -> ApiResult<Vendor> {
        self.api.post("vendors", Some(vendor)).await
    }

    /// Update an existing vendor
    pub async fn update_vendor(&self, id: Uuid, vendor: UpdateVendor) -> ApiResult<Vendor> {
        let endpoint = format!("vendors/{}", id);
        self.api.put(&endpoint, Some(vendor)).await
    }

    /// Delete a vendor
    pub async fn delete_vendor(&self, id: Uuid) -> ApiResult<()> {
        let endpoint = format!("vendors/{}", id);
        self.api.delete(&endpoint).await
    }

    /// Activate a vendor
    pub async fn activate_vendor(&self, id: Uuid) -> ApiResult<Vendor> {
        let endpoint = format!("vendors/{}/activate", id);
        self.api.put(&endpoint, None::<()>).await
    }

    /// Deactivate a vendor
    pub async fn deactivate_vendor(&self, id: Uuid) -> ApiResult<Vendor> {
        let endpoint = format!("vendors/{}/deactivate", id);
        self.api.put(&endpoint, None::<()>).await
    }

    /// Verify a vendor
    pub async fn verify_vendor(&self, id: Uuid) -> ApiResult<Vendor> {
        let endpoint = format!("vendors/{}/verify", id);
        self.api.put(&endpoint, None::<()>).await
    }

    /// Unverify a vendor
    pub async fn unverify_vendor(&self, id: Uuid) -> ApiResult<Vendor> {
        let endpoint = format!("vendors/{}/unverify", id);
        self.api.put(&endpoint, None::<()>).await
    }

    /// Search vendors
    pub async fn search_vendors(&self, search_term: &str, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<Vendor>> {
        let query = VendorQuery {
            search: Some(search_term.to_string()),
            page,
            limit,
            ..Default::default()
        };
        self.get_vendors(Some(query)).await
    }

    /// Get active vendors
    pub async fn get_active_vendors(&self, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<Vendor>> {
        let query = VendorQuery {
            active: Some(true),
            page,
            limit,
            ..Default::default()
        };
        self.get_vendors(Some(query)).await
    }

    /// Get verified vendors
    pub async fn get_verified_vendors(&self, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<Vendor>> {
        let query = VendorQuery {
            verified: Some(true),
            page,
            limit,
            ..Default::default()
        };
        self.get_vendors(Some(query)).await
    }

    /// Get vendor statistics
    pub async fn get_vendor_stats(&self) -> ApiResult<VendorStats> {
        self.api.get("vendors/stats").await
    }

    /// Get recent vendors
    pub async fn get_recent_vendors(&self, limit: Option<u32>) -> ApiResult<Vec<Vendor>> {
        let limit = limit.unwrap_or(10);
        let endpoint = format!("vendors/recent?limit={}", limit);
        self.api.get(&endpoint).await
    }

    /// Get vendor products
    pub async fn get_vendor_products(&self, vendor_id: Uuid, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<models::product::Product>> {
        let mut params = Vec::new();
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        let query_string = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        
        let endpoint = format!("vendors/{}/products{}", vendor_id, query_string);
        self.api.get(&endpoint).await
    }

    /// Get vendor orders
    pub async fn get_vendor_orders(&self, vendor_id: Uuid, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<models::order::Order>> {
        let mut params = Vec::new();
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        let query_string = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        
        let endpoint = format!("vendors/{}/orders{}", vendor_id, query_string);
        self.api.get(&endpoint).await
    }

    /// Get vendor sales statistics
    pub async fn get_vendor_sales_stats(&self, vendor_id: Uuid, from_date: Option<chrono::DateTime<chrono::Utc>>, to_date: Option<chrono::DateTime<chrono::Utc>>) -> ApiResult<VendorSalesStats> {
        let mut params = Vec::new();
        if let Some(from) = from_date {
            params.push(format!("from_date={}", from.to_rfc3339()));
        }
        if let Some(to) = to_date {
            params.push(format!("to_date={}", to.to_rfc3339()));
        }
        
        let query_string = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        
        let endpoint = format!("vendors/{}/sales/stats{}", vendor_id, query_string);
        self.api.get(&endpoint).await
    }

    /// Export vendors to CSV
    pub async fn export_vendors(&self, query: Option<VendorQuery>) -> ApiResult<String> {
        let query_string = query.unwrap_or_default().to_query_string();
        let endpoint = format!("vendors/export{}", query_string);
        self.api.get(&endpoint).await
    }

    /// Bulk update vendors
    pub async fn bulk_update(&self, updates: Vec<(Uuid, UpdateVendor)>) -> ApiResult<Vec<Vendor>> {
        let body = updates.into_iter().map(|(id, update)| {
            serde_json::json!({
                "id": id,
                "update": update
            })
        }).collect::<Vec<_>>();
        
        self.api.put("vendors/bulk", Some(body)).await
    }

    /// Upload vendor logo
    pub async fn upload_logo(&self, vendor_id: Uuid, logo_data: Vec<u8>, content_type: String) -> ApiResult<String> {
        // This would typically use a different endpoint for file uploads
        // For now, we'll use a placeholder implementation
        let endpoint = format!("vendors/{}/logo", vendor_id);
        let body = serde_json::json!({
            "logo_data": base64::encode(logo_data),
            "content_type": content_type
        });
        self.api.put(&endpoint, Some(body)).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorSalesStats {
    pub total_sales: f64,
    pub total_orders: u32,
    pub average_order_value: f64,
    pub top_products: Vec<TopProduct>,
    pub sales_by_month: Vec<MonthlySales>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopProduct {
    pub product_id: Uuid,
    pub product_name: String,
    pub sales_count: u32,
    pub total_revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlySales {
    pub month: String,
    pub sales: f64,
    pub orders: u32,
}

impl Default for VendorService {
    fn default() -> Self {
        Self::new(ApiService::default())
    }
}
