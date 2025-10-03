use models::user::User;
use super::api::{ApiService, ApiResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

pub struct AuthService {
    api: ApiService,
}

impl AuthService {
    pub fn new(api: ApiService) -> Self {
        Self { api }
    }

    /// Login with email and password
    pub async fn login(&self, request: LoginRequest) -> ApiResult<AuthResponse> {
        self.api.post("auth/login", Some(request)).await
    }

    /// Register a new user
    pub async fn register(&self, request: RegisterRequest) -> ApiResult<AuthResponse> {
        self.api.post("auth/register", Some(request)).await
    }

    /// Refresh authentication token
    pub async fn refresh_token(&self, request: RefreshTokenRequest) -> ApiResult<AuthResponse> {
        self.api.post("auth/refresh", Some(request)).await
    }

    /// Logout (invalidate token)
    pub async fn logout(&self) -> ApiResult<()> {
        self.api.post("auth/logout", None::<()>).await
    }

    /// Get current user profile
    pub async fn get_profile(&self) -> ApiResult<User> {
        self.api.get("auth/profile").await
    }

    /// Update user profile
    pub async fn update_profile(&self, user: User) -> ApiResult<User> {
        self.api.put("auth/profile", Some(user)).await
    }

    /// Change password
    pub async fn change_password(&self, request: ChangePasswordRequest) -> ApiResult<()> {
        self.api.put("auth/password", Some(request)).await
    }

    /// Request password reset
    pub async fn request_password_reset(&self, request: ResetPasswordRequest) -> ApiResult<()> {
        self.api.post("auth/password/reset", Some(request)).await
    }

    /// Confirm password reset with token
    pub async fn confirm_password_reset(&self, request: ConfirmResetPasswordRequest) -> ApiResult<()> {
        self.api.post("auth/password/confirm", Some(request)).await
    }

    /// Verify email address
    pub async fn verify_email(&self, token: String) -> ApiResult<()> {
        let endpoint = format!("auth/verify?token={}", token);
        self.api.post(&endpoint, None::<()>).await
    }

    /// Resend email verification
    pub async fn resend_verification(&self) -> ApiResult<()> {
        self.api.post("auth/verify/resend", None::<()>).await
    }

    /// Check if user is authenticated
    pub async fn check_auth(&self) -> ApiResult<User> {
        self.api.get("auth/check").await
    }

    /// Get user permissions
    pub async fn get_permissions(&self) -> ApiResult<Vec<String>> {
        self.api.get("auth/permissions").await
    }

    /// Check if user has specific permission
    pub async fn has_permission(&self, permission: &str) -> ApiResult<bool> {
        let endpoint = format!("auth/permissions/check?permission={}", permission);
        self.api.get(&endpoint).await
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new(ApiService::default())
    }
}

// Local storage utilities for token management
pub struct TokenStorage;

impl TokenStorage {
    const TOKEN_KEY: &'static str = "auth_token";
    const REFRESH_TOKEN_KEY: &'static str = "refresh_token";
    const USER_KEY: &'static str = "user_data";

    pub fn save_auth_data(auth_response: &AuthResponse) {
        if let Some(storage) = Self::get_local_storage() {
            let _ = storage.set_item(Self::TOKEN_KEY, &auth_response.token);
            let _ = storage.set_item(Self::REFRESH_TOKEN_KEY, &auth_response.refresh_token);
            if let Ok(user_json) = serde_json::to_string(&auth_response.user) {
                let _ = storage.set_item(Self::USER_KEY, &user_json);
            }
        }
    }

    pub fn get_token() -> Option<String> {
        Self::get_local_storage()
            .and_then(|storage| storage.get_item(Self::TOKEN_KEY).ok().flatten())
    }

    pub fn get_refresh_token() -> Option<String> {
        Self::get_local_storage()
            .and_then(|storage| storage.get_item(Self::REFRESH_TOKEN_KEY).ok().flatten())
    }

    pub fn get_user() -> Option<User> {
        Self::get_local_storage()
            .and_then(|storage| storage.get_item(Self::USER_KEY).ok().flatten())
            .and_then(|user_json| serde_json::from_str(&user_json).ok())
    }

    pub fn clear_auth_data() {
        if let Some(storage) = Self::get_local_storage() {
            let _ = storage.remove_item(Self::TOKEN_KEY);
            let _ = storage.remove_item(Self::REFRESH_TOKEN_KEY);
            let _ = storage.remove_item(Self::USER_KEY);
        }
    }

    pub fn is_authenticated() -> bool {
        Self::get_token().is_some()
    }

    fn get_local_storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok()?
    }
}
