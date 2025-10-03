use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Headers};
use js_sys::Promise;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ApiService {
    base_url: String,
    auth_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub code: Option<u16>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error: {}", self.message)
    }
}

impl std::error::Error for ApiError {}

pub type ApiResult<T> = Result<T, ApiError>;

impl ApiService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            auth_token: None,
        }
    }

    pub fn with_auth_token(mut self, token: String) -> Self {
        self.auth_token = Some(token);
        self
    }

    pub fn set_auth_token(&mut self, token: Option<String>) {
        self.auth_token = token;
    }

    pub async fn get<T>(&self, endpoint: &str) -> ApiResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.request("GET", endpoint, None::<()>).await
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: Option<B>) -> ApiResult<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        self.request("POST", endpoint, body).await
    }

    pub async fn put<T, B>(&self, endpoint: &str, body: Option<B>) -> ApiResult<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        self.request("PUT", endpoint, body).await
    }

    pub async fn delete<T>(&self, endpoint: &str) -> ApiResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.request("DELETE", endpoint, None::<()>).await
    }

    async fn request<T, B>(&self, method: &str, endpoint: &str, body: Option<B>) -> ApiResult<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint.trim_start_matches('/'));
        
        let mut opts = RequestInit::new();
        opts.method(method);
        opts.mode(RequestMode::Cors);

        // Set headers
        let headers = Headers::new().map_err(|_| ApiError {
            message: "Failed to create headers".to_string(),
            code: None,
        })?;

        headers.set("Content-Type", "application/json").map_err(|_| ApiError {
            message: "Failed to set content type header".to_string(),
            code: None,
        })?;

        if let Some(token) = &self.auth_token {
            headers.set("Authorization", &format!("Bearer {}", token)).map_err(|_| ApiError {
                message: "Failed to set authorization header".to_string(),
                code: None,
            })?;
        }

        opts.headers(&headers);

        // Set body if provided
        if let Some(body) = body {
            let body_str = serde_json::to_string(&body).map_err(|e| ApiError {
                message: format!("Failed to serialize request body: {}", e),
                code: None,
            })?;
            opts.body(Some(&JsValue::from_str(&body_str)));
        }

        let request = Request::new_with_str_and_init(&url, &opts).map_err(|_| ApiError {
            message: "Failed to create request".to_string(),
            code: None,
        })?;

        let window = web_sys::window().ok_or_else(|| ApiError {
            message: "No global window object".to_string(),
            code: None,
        })?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| ApiError {
                message: "Network request failed".to_string(),
                code: None,
            })?;

        let resp: Response = resp_value.dyn_into().map_err(|_| ApiError {
            message: "Failed to cast response".to_string(),
            code: None,
        })?;

        let status = resp.status();
        let text = JsFuture::from(resp.text().map_err(|_| ApiError {
            message: "Failed to get response text".to_string(),
            code: None,
        })?)
        .await
        .map_err(|_| ApiError {
            message: "Failed to read response text".to_string(),
            code: None,
        })?;

        let text_str = text.as_string().ok_or_else(|| ApiError {
            message: "Response text is not a string".to_string(),
            code: None,
        })?;

        if status >= 200 && status < 300 {
            serde_json::from_str(&text_str).map_err(|e| ApiError {
                message: format!("Failed to deserialize response: {}", e),
                code: Some(status),
            })
        } else {
            // Try to parse error response
            let error_msg = if let Ok(error_response) = serde_json::from_str::<HashMap<String, String>>(&text_str) {
                error_response.get("message")
                    .or_else(|| error_response.get("error"))
                    .cloned()
                    .unwrap_or_else(|| format!("HTTP {} error", status))
            } else {
                format!("HTTP {} error: {}", status, text_str)
            };

            Err(ApiError {
                message: error_msg,
                code: Some(status),
            })
        }
    }
}

// Default API service instance
impl Default for ApiService {
    fn default() -> Self {
        // In a real application, this would come from environment variables or configuration
        Self::new("http://localhost:8080/api")
    }
}
