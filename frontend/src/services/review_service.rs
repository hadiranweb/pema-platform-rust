
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
    fn get_token() -> anyhow::Result<String> {
        LocalStorage::get(TOKEN_KEY).map_err(|_| anyhow::anyhow!("Token not found"))
    }

    pub async fn create_review(create_review: CreateReview) -> anyhow::Result<Review> {
        let token = Self::get_token()?;
        let response = Request::post("/api/reviews")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&create_review)?
            .send()
            .await?;

        if response.ok() {
            let review: Review = response.json().await?;
            Ok(review)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Create review failed: {}", error_text))
        }
    }

    pub async fn get_review_by_id(review_id: String) -> anyhow::Result<Review> {
        let token = Self::get_token()?;
        let response = Request::get(&format!("/api/reviews/{}", review_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            let review: Review = response.json().await?;
            Ok(review)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to fetch review: {}", error_text))
        }
    }

    pub async fn get_reviews_for_product(product_id: String) -> anyhow::Result<Vec<Review>> {
        let token = Self::get_token()?;
        let response = Request::get(&format!("/api/reviews/product/{}", product_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            let reviews: Vec<Review> = response.json().await?;
            Ok(reviews)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to fetch product reviews: {}", error_text))
        }
    }

    pub async fn get_my_reviews() -> anyhow::Result<Vec<Review>> {
        let token = Self::get_token()?;
        let response = Request::get("/api/reviews/me")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            let reviews: Vec<Review> = response.json().await?;
            Ok(reviews)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to fetch my reviews: {}", error_text))
        }
    }

    pub async fn update_review(review_id: String, update_review: UpdateReview) -> anyhow::Result<Review> {
        let token = Self::get_token()?;
        let response = Request::put(&format!("/api/reviews/{}", review_id))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&update_review)?
            .send()
            .await?;

        if response.ok() {
            let review: Review = response.json().await?;
            Ok(review)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Update review failed: {}", error_text))
        }
    }

    pub async fn delete_review(review_id: String) -> anyhow::Result<()> {
        let token = Self::get_token()?;
        let response = Request::delete(&format!("/api/reviews/{}", review_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Delete review failed: {}", error_text))
        }
    }
}

