use models::user::User;
use models::pagination::PaginatedResponse;
use super::api::{ApiService, ApiResult};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub role: Option<String>,
    pub active: Option<bool>,
}

impl Default for UserQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            search: None,
            role: None,
            active: None,
        }
    }
}

impl UserQuery {
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
        if let Some(role) = &self.role {
            params.push(format!("role={}", urlencoding::encode(role)));
        }
        if let Some(active) = self.active {
            params.push(format!("active={}", active));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub total_users: u32,
    pub active_users: u32,
    pub new_users_today: u32,
    pub new_users_this_week: u32,
    pub new_users_this_month: u32,
    pub users_by_role: std::collections::HashMap<String, u32>,
}

pub struct UserService {
    api: ApiService,
}

impl UserService {
    pub fn new(api: ApiService) -> Self {
        Self { api }
    }

    /// Get all users with optional filtering and pagination
    pub async fn get_users(&self, query: Option<UserQuery>) -> ApiResult<PaginatedResponse<User>> {
        let query_string = query.unwrap_or_default().to_query_string();
        let endpoint = format!("users{}", query_string);
        self.api.get(&endpoint).await
    }

    /// Get a single user by ID
    pub async fn get_user(&self, id: Uuid) -> ApiResult<User> {
        let endpoint = format!("users/{}", id);
        self.api.get(&endpoint).await
    }

    /// Create a new user
    pub async fn create_user(&self, user: CreateUser) -> ApiResult<User> {
        self.api.post("users", Some(user)).await
    }

    /// Update an existing user
    pub async fn update_user(&self, id: Uuid, user: UpdateUser) -> ApiResult<User> {
        let endpoint = format!("users/{}", id);
        self.api.put(&endpoint, Some(user)).await
    }

    /// Delete a user
    pub async fn delete_user(&self, id: Uuid) -> ApiResult<()> {
        let endpoint = format!("users/{}", id);
        self.api.delete(&endpoint).await
    }

    /// Activate a user
    pub async fn activate_user(&self, id: Uuid) -> ApiResult<User> {
        let endpoint = format!("users/{}/activate", id);
        self.api.put(&endpoint, None::<()>).await
    }

    /// Deactivate a user
    pub async fn deactivate_user(&self, id: Uuid) -> ApiResult<User> {
        let endpoint = format!("users/{}/deactivate", id);
        self.api.put(&endpoint, None::<()>).await
    }

    /// Search users
    pub async fn search_users(&self, search_term: &str, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<User>> {
        let query = UserQuery {
            search: Some(search_term.to_string()),
            page,
            limit,
            ..Default::default()
        };
        self.get_users(Some(query)).await
    }

    /// Get users by role
    pub async fn get_users_by_role(&self, role: &str, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<User>> {
        let query = UserQuery {
            role: Some(role.to_string()),
            page,
            limit,
            ..Default::default()
        };
        self.get_users(Some(query)).await
    }

    /// Get user statistics
    pub async fn get_user_stats(&self) -> ApiResult<UserStats> {
        self.api.get("users/stats").await
    }

    /// Get recent users
    pub async fn get_recent_users(&self, limit: Option<u32>) -> ApiResult<Vec<User>> {
        let limit = limit.unwrap_or(10);
        let endpoint = format!("users/recent?limit={}", limit);
        self.api.get(&endpoint).await
    }

    /// Change user role
    pub async fn change_user_role(&self, id: Uuid, role: String) -> ApiResult<User> {
        let endpoint = format!("users/{}/role", id);
        let body = serde_json::json!({ "role": role });
        self.api.put(&endpoint, Some(body)).await
    }

    /// Reset user password (admin only)
    pub async fn reset_user_password(&self, id: Uuid, new_password: String) -> ApiResult<()> {
        let endpoint = format!("users/{}/password/reset", id);
        let body = serde_json::json!({ "password": new_password });
        self.api.put(&endpoint, Some(body)).await
    }

    /// Get user activity log
    pub async fn get_user_activity(&self, id: Uuid, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<UserActivity>> {
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
        
        let endpoint = format!("users/{}/activity{}", id, query_string);
        self.api.get(&endpoint).await
    }

    /// Export users to CSV
    pub async fn export_users(&self, query: Option<UserQuery>) -> ApiResult<String> {
        let query_string = query.unwrap_or_default().to_query_string();
        let endpoint = format!("users/export{}", query_string);
        self.api.get(&endpoint).await
    }

    /// Bulk update users
    pub async fn bulk_update(&self, updates: Vec<(Uuid, UpdateUser)>) -> ApiResult<Vec<User>> {
        let body = updates.into_iter().map(|(id, update)| {
            serde_json::json!({
                "id": id,
                "update": update
            })
        }).collect::<Vec<_>>();
        
        self.api.put("users/bulk", Some(body)).await
    }

    /// Get available user roles
    pub async fn get_roles(&self) -> ApiResult<Vec<String>> {
        self.api.get("users/roles").await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub description: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Default for UserService {
    fn default() -> Self {
        Self::new(ApiService::default())
    }
}
