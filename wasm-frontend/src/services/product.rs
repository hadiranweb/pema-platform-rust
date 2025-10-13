use models::product::{Product, CreateProduct, UpdateProduct};
use models::pagination::PaginatedResponse;
use super::api::{ApiService, ApiResult};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub category: Option<String>,
    pub vendor_id: Option<Uuid>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub in_stock: Option<bool>,
}

impl Default for ProductQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            search: None,
            category: None,
            vendor_id: None,
            min_price: None,
            max_price: None,
            in_stock: None,
        }
    }
}

impl ProductQuery {
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
        if let Some(category) = &self.category {
            params.push(format!("category={}", urlencoding::encode(category)));
        }
        if let Some(vendor_id) = self.vendor_id {
            params.push(format!("vendor_id={}", vendor_id));
        }
        if let Some(min_price) = self.min_price {
            params.push(format!("min_price={}", min_price));
        }
        if let Some(max_price) = self.max_price {
            params.push(format!("max_price={}", max_price));
        }
        if let Some(in_stock) = self.in_stock {
            params.push(format!("in_stock={}", in_stock));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

pub struct ProductService {
    api: ApiService,
}

impl ProductService {
    pub fn new(api: ApiService) -> Self {
        Self { api }
    }

    /// Get all products with optional filtering and pagination
    pub async fn get_products(&self, query: Option<ProductQuery>) -> ApiResult<PaginatedResponse<Product>> {
        let query_string = query.unwrap_or_default().to_query_string();
        let endpoint = format!("products{}", query_string);
        self.api.get(&endpoint).await
    }

    /// Get a single product by ID
    pub async fn get_product(&self, id: Uuid) -> ApiResult<Product> {
        let endpoint = format!("products/{}", id);
        self.api.get(&endpoint).await
    }

    /// Create a new product
    pub async fn create_product(&self, product: CreateProduct) -> ApiResult<Product> {
        self.api.post("products", Some(product)).await
    }

    /// Update an existing product
    pub async fn update_product(&self, id: Uuid, product: UpdateProduct) -> ApiResult<Product> {
        let endpoint = format!("products/{}", id);
        self.api.put(&endpoint, Some(product)).await
    }

    /// Delete a product
    pub async fn delete_product(&self, id: Uuid) -> ApiResult<()> {
        let endpoint = format!("products/{}", id);
        self.api.delete(&endpoint).await
    }

    /// Get products by category
    pub async fn get_products_by_category(&self, category: &str, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<Product>> {
        let query = ProductQuery {
            category: Some(category.to_string()),
            page,
            limit,
            ..Default::default()
        };
        self.get_products(Some(query)).await
    }

    /// Get products by vendor
    pub async fn get_products_by_vendor(&self, vendor_id: Uuid, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<Product>> {
        let query = ProductQuery {
            vendor_id: Some(vendor_id),
            page,
            limit,
            ..Default::default()
        };
        self.get_products(Some(query)).await
    }

    /// Search products
    pub async fn search_products(&self, search_term: &str, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<Product>> {
        let query = ProductQuery {
            search: Some(search_term.to_string()),
            page,
            limit,
            ..Default::default()
        };
        self.get_products(Some(query)).await
    }

    /// Get low stock products
    pub async fn get_low_stock_products(&self, threshold: i32) -> ApiResult<Vec<Product>> {
        let endpoint = format!("products/low-stock?threshold={}", threshold);
        self.api.get(&endpoint).await
    }

    /// Get product categories
    pub async fn get_categories(&self) -> ApiResult<Vec<String>> {
        self.api.get("products/categories").await
    }

    /// Update product stock
    pub async fn update_stock(&self, id: Uuid, stock: i32) -> ApiResult<Product> {
        let endpoint = format!("products/{}/stock", id);
        let body = serde_json::json!({ "stock": stock });
        self.api.put(&endpoint, Some(body)).await
    }

    /// Bulk update products
    pub async fn bulk_update(&self, updates: Vec<(Uuid, UpdateProduct)>) -> ApiResult<Vec<Product>> {
        let body = updates.into_iter().map(|(id, update)| {
            serde_json::json!({
                "id": id,
                "update": update
            })
        }).collect::<Vec<_>>();
        
        self.api.put("products/bulk", Some(body)).await
    }
}

impl Default for ProductService {
    fn default() -> Self {
        Self::new(ApiService::default())
    }
}
