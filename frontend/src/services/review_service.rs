
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use gloo_storage::{LocalStorage, Storage};

use crate::utils::constants::TOKEN_KEY;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Review {
    pub id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateReview {
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateReview {
    pub rating: Option<i32>,
    pub comment: Option<String>,
}

pub struct ReviewService;

impl ReviewService {
    fn get_token() -> Result<String, String> {
        LocalStorage::get(TOKEN_KEY).map_err(|_| "Token not found".to_string())
    }

    pub async fn create_review(create_review: CreateReview) -> Result<Review, String> {
        let token = Self::get_token()?;
        let response = Request::post("/api/reviews")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&create_review)
            .map_err(|e| format!("Failed to serialize create review request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse review after creation: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Create review failed: {}", error_text))
        }
    }

    pub async fn get_review_by_id(review_id: String) -> Result<Review, String> {
        let token = Self::get_token()?;
        let response = Request::get(&format!("/api/reviews/{}", review_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse review: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch review: {}", error_text))
        }
    }

    pub async fn get_reviews_for_product(product_id: String) -> Result<Vec<Review>, String> {
        let token = Self::get_token()?;
        let response = Request::get(&format!("/api/reviews/product/{}", product_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse product reviews: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch product reviews: {}", error_text))
        }
    }

    pub async fn get_my_reviews() -> Result<Vec<Review>, String> {
        let token = Self::get_token()?;
        let response = Request::get("/api/reviews/me")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse my reviews: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch my reviews: {}", error_text))
        }
    }

    pub async fn update_review(review_id: String, update_review: UpdateReview) -> Result<Review, String> {
        let token = Self::get_token()?;
        let response = Request::put(&format!("/api/reviews/{}", review_id))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&update_review)
            .map_err(|e| format!("Failed to serialize update review request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse review after update: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Update review failed: {}", error_text))
        }
    }

    pub async fn delete_review(review_id: String) -> Result<(), String> {
        let token = Self::get_token()?;
        let response = Request::delete(&format!("/api/reviews/{}", review_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Delete review failed: {}", error_text))
        }
    }
}

