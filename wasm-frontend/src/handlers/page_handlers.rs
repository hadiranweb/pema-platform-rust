use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tera::{Tera, Context};
use uuid::Uuid;
use models::user::User;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub email_or_phone: String,
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub email_or_phone: String,
    pub code: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email_or_phone: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub phone: String,
    pub code: String,
}

#[derive(Deserialize)]
pub struct SocialLoginRequest {
    pub provider: String,
    pub access_token: String,
    pub user_info: SocialUserInfo,
}

#[derive(Deserialize)]
pub struct SocialUserInfo {
    pub id: String,
    pub email: Option<String>,
    pub name: String,
    pub avatar_url: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub exists: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<Uuid>,
    pub token: Option<String>,
}

pub async fn landing_page(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = Context::new();
    context.insert("title", "PEMA - پلتفرم نوین تجارت الکترونیک");
    
    let html = tera.render("landing.html", &context)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn check_user(
    pool: web::Data<PgPool>,
    req: web::Json<AuthRequest>,
) -> Result<HttpResponse> {
    // Check if user exists by email or phone
    let user_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 OR username = $1)"
    )
    .bind(&req.email_or_phone)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(false);

    let response = if user_exists {
        AuthResponse {
            exists: true,
            message: "کاربر موجود است. لطفاً رمز عبور خود را وارد کنید.".to_string(),
        }
    } else {
        // Send verification code (simulate)
        // In real implementation, integrate with notification service
        AuthResponse {
            exists: false,
            message: "کد تایید به شماره/ایمیل شما ارسال شد.".to_string(),
        }
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn verify_code(req: web::Json<VerifyRequest>) -> Result<HttpResponse> {
    // Simulate code verification (in real app, check against stored code)
    // In production, this would verify against a code stored in Redis or database
    if req.code == "1234" {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "کد تایید صحیح است."
        })))
    } else {
        Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "کد تایید نادرست است."
        })))
    }
}

pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    // Verify user credentials
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1 OR username = $1"
    )
    .bind(&req.email_or_phone)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if let Some(user) = user {
        // In real app, verify password hash using bcrypt
        // For now, simulate successful login
        let response = LoginResponse {
            success: true,
            message: "ورود موفقیت‌آمیز بود.".to_string(),
            user_id: Some(user.id),
            token: Some(format!("token_{}", user.id)),
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = LoginResponse {
            success: false,
            message: "نام کاربری یا رمز عبور نادرست است.".to_string(),
            user_id: None,
            token: None,
        };
        Ok(HttpResponse::Unauthorized().json(response))
    }
}

pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    // Verify code first
    if req.code != "1234" {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "کد تایید نادرست است."
        })));
    }

    // Check if user already exists
    let existing_user = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 OR username = $2)"
    )
    .bind(&req.email)
    .bind(&req.username)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if existing_user {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "کاربر با این ایمیل یا نام کاربری قبلاً ثبت‌نام کرده است."
        })));
    }

    // Create new user
    let user_id = Uuid::new_v4();
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, email, password_hash, created_at, updated_at) 
         VALUES ($1, $2, $3, $4, NOW(), NOW()) RETURNING *"
    )
    .bind(user_id)
    .bind(&req.username)
    .bind(&req.email)
    .bind("temp_password_hash") // In real app, hash the password
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let response = LoginResponse {
        success: true,
        message: "ثبت‌نام با موفقیت انجام شد.".to_string(),
        user_id: Some(user.id),
        token: Some(format!("token_{}", user.id)),
    };

    Ok(HttpResponse::Created().json(response))
}

pub async fn social_login(
    pool: web::Data<PgPool>,
    req: web::Json<SocialLoginRequest>,
) -> Result<HttpResponse> {
    // Verify the social login token (in real app, verify with the provider)
    // For now, simulate successful verification
    
    let default_email = format!("{}@{}.com", req.user_info.id, req.provider);
    let email = req.user_info.email.as_ref().unwrap_or(&default_email);
    
    // Check if user exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let user = if let Some(user) = existing_user {
        // User exists, update last login
        sqlx::query(
            "UPDATE users SET updated_at = NOW() WHERE id = $1"
        )
        .bind(user.id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        
        user
    } else {
        // Create new user from social login
        let user_id = Uuid::new_v4();
        sqlx::query_as::<_, User>(
            "INSERT INTO users (id, username, email, password_hash, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, NOW(), NOW()) RETURNING *"
        )
        .bind(user_id)
        .bind(&req.user_info.name)
        .bind(email)
        .bind("social_login") // Mark as social login
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    };

    let response = LoginResponse {
        success: true,
        message: format!("ورود با {} موفقیت‌آمیز بود.", match req.provider.as_str() {
            "google" => "گوگل",
            "github" => "گیت‌هاب",
            "linkedin" => "لینکدین",
            "meta" => "متا",
            _ => "شبکه اجتماعی"
        }),
        user_id: Some(user.id),
        token: Some(format!("token_{}_{}", req.provider, user.id)),
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_products() -> Result<HttpResponse> {
    // Call product-catalog service
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8081/api/products?page=1&limit=6")
        .send()
        .await;

    match response {
        Ok(resp) => {
            let products: serde_json::Value = resp.json().await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            Ok(HttpResponse::Ok().json(products))
        }
        Err(_) => {
            // Return enhanced mock data if service is not available
            let mock_products = serde_json::json!({
                "items": [
                    {
                        "id": "550e8400-e29b-41d4-a716-446655440001",
                        "name": "لپ‌تاپ گیمینگ ASUS ROG",
                        "description": "لپ‌تاپ قدرتمند با پردازنده Intel Core i7 و کارت گرافیک RTX 4060",
                        "price": 45000000,
                        "stock_quantity": 5,
                        "category": "لپ‌تاپ",
                        "image_url": "/static/images/laptop.jpg"
                    },
                    {
                        "id": "550e8400-e29b-41d4-a716-446655440002",
                        "name": "گوشی هوشمند Samsung Galaxy S24",
                        "description": "گوشی پرچمدار سامسونگ با دوربین 200 مگاپیکسل و نمایشگر Dynamic AMOLED",
                        "price": 28000000,
                        "stock_quantity": 12,
                        "category": "گوشی موبایل",
                        "image_url": "/static/images/phone.jpg"
                    },
                    {
                        "id": "550e8400-e29b-41d4-a716-446655440003",
                        "name": "هدفون بی‌سیم Sony WH-1000XM5",
                        "description": "هدفون با کیفیت صدای Hi-Res و قابلیت حذف نویز فعال",
                        "price": 8500000,
                        "stock_quantity": 20,
                        "category": "صوتی",
                        "image_url": "/static/images/headphones.jpg"
                    },
                    {
                        "id": "550e8400-e29b-41d4-a716-446655440004",
                        "name": "ساعت هوشمند Apple Watch Series 9",
                        "description": "ساعت هوشمند اپل با سنسور اکسیژن خون و GPS دقیق",
                        "price": 15000000,
                        "stock_quantity": 8,
                        "category": "پوشیدنی",
                        "image_url": "/static/images/watch.jpg"
                    },
                    {
                        "id": "550e8400-e29b-41d4-a716-446655440005",
                        "name": "تبلت iPad Pro 12.9 اینچ",
                        "description": "تبلت حرفه‌ای اپل با تراشه M2 و نمایشگر Liquid Retina XDR",
                        "price": 35000000,
                        "stock_quantity": 3,
                        "category": "تبلت",
                        "image_url": "/static/images/tablet.jpg"
                    },
                    {
                        "id": "550e8400-e29b-41d4-a716-446655440006",
                        "name": "کیبورد مکانیکی Logitech MX Keys",
                        "description": "کیبورد بی‌سیم با کلیدهای مکانیکی و نورپردازی هوشمند",
                        "price": 3200000,
                        "stock_quantity": 15,
                        "category": "جانبی",
                        "image_url": "/static/images/keyboard.jpg"
                    }
                ],
                "total_items": 6,
                "current_page": 1,
                "total_pages": 1,
                "limit": 6
            });
            Ok(HttpResponse::Ok().json(mock_products))
        }
    }
}

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "landing-page",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
