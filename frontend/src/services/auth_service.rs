use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub otp_code: Option<String>,
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
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
}

pub struct AuthService;

impl AuthService {
    pub async fn login(request: LoginRequest) -> Result<(String, Uuid, String, String)> {
        let response = Request::post("/api/auth/login")
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            Ok((auth_response.token, auth_response.user_id, auth_response.username, auth_response.email))
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
            Ok((auth_response.token, auth_response.user_id, auth_response.username, auth_response.email))
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Registration failed: {}", error_text))
        }
    }

    pub async fn generate_otp(token: &str) -> Result<String> {
        let response = Request::post("/api/auth/otp/generate")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            let json_response: serde_json::Value = response.json().await?;
            let otp_code = json_response["otp_code"].as_str().unwrap_or_default().to_string();
            Ok(otp_code)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to generate OTP: {}", error_text))
        }
    }
}

