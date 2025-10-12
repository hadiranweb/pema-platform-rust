use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

pub struct AuthService;

impl AuthService {
    pub async fn login(request: LoginRequest) -> Result<String> {
        let response = Request::post("/api/auth/login")
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            // Assuming the backend returns user_id, username, and email along with the token
            // For now, we'll use dummy values or parse them from the token if available
            // This needs to be aligned with the backend API response.
            // For demonstration, let's assume the token itself contains enough info or we get it from another endpoint.
            // For now, let's just return the token and a dummy user_id, username, email.
            // In a real app, you'd decode the JWT or fetch user info.
            let user_id = Uuid::new_v4(); // Placeholder
            let username = "testuser".to_string(); // Placeholder
            let email = request.email.clone(); // Placeholder
            Ok((auth_response.token, user_id, username, email))
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Login failed: {}", error_text))
        }
    }

    pub async fn register(request: RegisterRequest) -> Result<(String, Uuid, String, String)> {
        let response = Request::post("/api/auth/register")
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            // For demonstration, assume backend returns user_id, username, and email.
            // In a real app, you'd decode the JWT or fetch user info.
            let user_id = Uuid::new_v4(); // Placeholder
            let username = request.username.clone(); // Placeholder
            let email = request.email.clone(); // Placeholder
            Ok((auth_response.token, user_id, username, email))
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Registration failed: {}", error_text))
        }
    }
}

