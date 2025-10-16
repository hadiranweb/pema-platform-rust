use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub identifier: String, // email or phone
    pub password: String,
    pub login_type: LoginType,
    pub otp_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub identifier: String, // email or phone
    pub name: String,
    pub password: String,
    pub register_type: RegisterType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OtpRequest {
    pub identifier: String,
    pub otp_type: OtpType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyOtpRequest {
    pub identifier: String,
    pub otp: String,
    pub otp_type: OtpType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LoginType {
    Email,
    Phone,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RegisterType {
    Email,
    Phone,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OtpType {
    Email,
    Sms,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
    pub user: Option<UserInfo>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub verified: bool,
}

#[derive(Clone, Debug)]
pub struct AuthService {
    base_url: String,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            base_url: "/api/auth".to_string(),
        }
    }

    // ورود با ایمیل
    pub async fn login_with_email(&self, email: String, password: String) -> Result<AuthResponse> {
        let request = LoginRequest {
            identifier: email,
            password,
            login_type: LoginType::Email,
            otp_code: None,
        };

        let response = Request::post(&format!("{}/login", self.base_url))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            Ok(auth_response)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("ورود ناموفق: {}", error_text))
        }
    }

    // ثبت‌نام با ایمیل
    pub async fn register_with_email(&self, email: String, password: String, name: String) -> Result<AuthResponse> {
        let request = RegisterRequest {
            identifier: email,
            password,
            name,
            register_type: RegisterType::Email,
        };

        let response = Request::post(&format!("{}/register", self.base_url))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            Ok(auth_response)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("ثبت‌نام ناموفق: {}", error_text))
        }
    }

    // ورود با شماره تلفن
    pub async fn login_with_phone(&self, phone: String, password: String) -> Result<AuthResponse> {
        let request = LoginRequest {
            identifier: phone,
            password,
            login_type: LoginType::Phone,
            otp_code: None,
        };

        let response = Request::post(&format!("{}/login", self.base_url))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            Ok(auth_response)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("ورود ناموفق: {}", error_text))
        }
    }

    // ثبت‌نام با شماره تلفن
    pub async fn register_with_phone(&self, phone: String, password: String, name: String) -> Result<AuthResponse> {
        let request = RegisterRequest {
            identifier: phone,
            password,
            name,
            register_type: RegisterType::Phone,
        };

        let response = Request::post(&format!("{}/register", self.base_url))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            Ok(auth_response)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("ثبت‌نام ناموفق: {}", error_text))
        }
    }

    // ارسال کد تأیید ایمیل
    pub async fn send_email_otp(&self, email: String) -> Result<AuthResponse> {
        let request = OtpRequest {
            identifier: email,
            otp_type: OtpType::Email,
        };

        let response = Request::post(&format!("{}/send-otp", self.base_url))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            Ok(auth_response)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("ارسال کد ناموفق: {}", error_text))
        }
    }

    // ارسال کد تأیید پیامک
    pub async fn send_sms_otp(&self, phone: String) -> Result<AuthResponse> {
        let request = OtpRequest {
            identifier: phone,
            otp_type: OtpType::Sms,
        };

        let response = Request::post(&format!("{}/send-otp", self.base_url))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            Ok(auth_response)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("ارسال کد ناموفق: {}", error_text))
        }
    }

    // تأیید کد ایمیل
    pub async fn verify_email_otp(&self, email: String, otp: String) -> Result<String> {
        let request = VerifyOtpRequest {
            identifier: email,
            otp,
            otp_type: OtpType::Email,
        };

        let response = Request::post(&format!("{}/verify-otp", self.base_url))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            if auth_response.success {
                auth_response.token.ok_or_else(|| anyhow::anyhow!("توکن دریافت نشد"))
            } else {
                Err(anyhow::anyhow!("{}", auth_response.message))
            }
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("تأیید کد ناموفق: {}", error_text))
        }
    }

    // تأیید کد پیامک
    pub async fn verify_sms_otp(&self, phone: String, otp: String) -> Result<String> {
        let request = VerifyOtpRequest {
            identifier: phone,
            otp,
            otp_type: OtpType::Sms,
        };

        let response = Request::post(&format!("{}/verify-otp", self.base_url))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            if auth_response.success {
                auth_response.token.ok_or_else(|| anyhow::anyhow!("توکن دریافت نشد"))
            } else {
                Err(anyhow::anyhow!("{}", auth_response.message))
            }
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("تأیید کد ناموفق: {}", error_text))
        }
    }

    // خروج از حساب کاربری
    pub async fn logout(&self, token: &str) -> Result<()> {
        let response = Request::post(&format!("{}/logout", self.base_url))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("خروج ناموفق: {}", error_text))
        }
    }

    // بررسی وضعیت احراز هویت
    pub async fn check_auth_status(&self, token: &str) -> Result<UserInfo> {
        let response = Request::get(&format!("{}/me", self.base_url))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            if auth_response.success {
                auth_response.user.ok_or_else(|| anyhow::anyhow!("اطلاعات کاربر دریافت نشد"))
            } else {
                Err(anyhow::anyhow!("{}", auth_response.message))
            }
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("بررسی وضعیت ناموفق: {}", error_text))
        }
    }

    // تجدید توکن
    pub async fn refresh_token(&self, token: &str) -> Result<String> {
        let response = Request::post(&format!("{}/refresh", self.base_url))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            let auth_response: AuthResponse = response.json().await?;
            if auth_response.success {
                auth_response.token.ok_or_else(|| anyhow::anyhow!("توکن جدید دریافت نشد"))
            } else {
                Err(anyhow::anyhow!("{}", auth_response.message))
            }
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("تجدید توکن ناموفق: {}", error_text))
        }
    }

    // متدهای کمکی برای ذخیره و بازیابی توکن
    pub fn store_token(&self, token: &str) {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("auth_token", token);
        }
    }

    pub fn get_stored_token(&self) -> Option<String> {
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item("auth_token").ok().flatten())
    }

    pub fn clear_stored_token(&self) {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.remove_item("auth_token");
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.get_stored_token().is_some()
    }
}

