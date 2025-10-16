use serde::{Deserialize, Serialize};
use std::fmt;

/// مدیریت خطاهای سیستم با رویکرد توسعه‌محور
/// این ماژول خطاهای مختلف سیستم را به صورت یکپارچه مدیریت می‌کند

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PemaError {
    // Authentication Errors
    AuthenticationFailed {
        reason: String,
        code: String,
    },
    InvalidCredentials {
        field: String,
        message: String,
    },
    TokenExpired {
        expired_at: String,
    },
    TokenInvalid {
        reason: String,
    },
    UnauthorizedAccess {
        required_permission: String,
        user_role: String,
    },

    // Validation Errors
    ValidationError {
        field: String,
        message: String,
        value: Option<String>,
    },
    InvalidInput {
        parameter: String,
        expected: String,
        received: String,
    },
    MissingRequiredField {
        field: String,
    },

    // Database Errors
    DatabaseError {
        operation: String,
        message: String,
        code: Option<String>,
    },
    RecordNotFound {
        entity: String,
        id: String,
    },
    DuplicateRecord {
        entity: String,
        field: String,
        value: String,
    },
    DatabaseConnectionFailed {
        reason: String,
    },

    // Network Errors
    NetworkError {
        url: String,
        status_code: Option<u16>,
        message: String,
    },
    TimeoutError {
        operation: String,
        timeout_seconds: u64,
    },
    ServiceUnavailable {
        service: String,
        retry_after: Option<u64>,
    },

    // Business Logic Errors
    InsufficientFunds {
        required: String,
        available: String,
        currency: String,
    },
    OrderProcessingFailed {
        order_id: String,
        reason: String,
    },
    PaymentFailed {
        payment_id: String,
        reason: String,
        error_code: Option<String>,
    },
    InventoryError {
        product_id: String,
        requested: u32,
        available: u32,
    },

    // System Errors
    InternalServerError {
        message: String,
        trace_id: Option<String>,
    },
    ConfigurationError {
        parameter: String,
        message: String,
    },
    ResourceExhausted {
        resource: String,
        limit: String,
    },
    MaintenanceMode {
        estimated_duration: Option<String>,
    },

    // File/Storage Errors
    FileNotFound {
        path: String,
    },
    FileAccessDenied {
        path: String,
        operation: String,
    },
    StorageError {
        operation: String,
        message: String,
    },

    // Rate Limiting
    RateLimitExceeded {
        limit: u32,
        window_seconds: u64,
        retry_after: u64,
    },

    // Generic Error
    Unknown {
        message: String,
        source: Option<String>,
    },
}

impl PemaError {
    /// ایجاد خطای احراز هویت
    pub fn auth_failed(reason: &str) -> Self {
        Self::AuthenticationFailed {
            reason: reason.to_string(),
            code: "AUTH_FAILED".to_string(),
        }
    }

    /// ایجاد خطای اعتبارسنجی
    pub fn validation_error(field: &str, message: &str) -> Self {
        Self::ValidationError {
            field: field.to_string(),
            message: message.to_string(),
            value: None,
        }
    }

    /// ایجاد خطای دیتابیس
    pub fn database_error(operation: &str, message: &str) -> Self {
        Self::DatabaseError {
            operation: operation.to_string(),
            message: message.to_string(),
            code: None,
        }
    }

    /// ایجاد خطای شبکه
    pub fn network_error(url: &str, message: &str) -> Self {
        Self::NetworkError {
            url: url.to_string(),
            status_code: None,
            message: message.to_string(),
        }
    }

    /// دریافت کد خطا
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::AuthenticationFailed { .. } => "AUTH_FAILED",
            Self::InvalidCredentials { .. } => "INVALID_CREDENTIALS",
            Self::TokenExpired { .. } => "TOKEN_EXPIRED",
            Self::TokenInvalid { .. } => "TOKEN_INVALID",
            Self::UnauthorizedAccess { .. } => "UNAUTHORIZED",
            Self::ValidationError { .. } => "VALIDATION_ERROR",
            Self::InvalidInput { .. } => "INVALID_INPUT",
            Self::MissingRequiredField { .. } => "MISSING_FIELD",
            Self::DatabaseError { .. } => "DATABASE_ERROR",
            Self::RecordNotFound { .. } => "RECORD_NOT_FOUND",
            Self::DuplicateRecord { .. } => "DUPLICATE_RECORD",
            Self::DatabaseConnectionFailed { .. } => "DB_CONNECTION_FAILED",
            Self::NetworkError { .. } => "NETWORK_ERROR",
            Self::TimeoutError { .. } => "TIMEOUT",
            Self::ServiceUnavailable { .. } => "SERVICE_UNAVAILABLE",
            Self::InsufficientFunds { .. } => "INSUFFICIENT_FUNDS",
            Self::OrderProcessingFailed { .. } => "ORDER_PROCESSING_FAILED",
            Self::PaymentFailed { .. } => "PAYMENT_FAILED",
            Self::InventoryError { .. } => "INVENTORY_ERROR",
            Self::InternalServerError { .. } => "INTERNAL_SERVER_ERROR",
            Self::ConfigurationError { .. } => "CONFIGURATION_ERROR",
            Self::ResourceExhausted { .. } => "RESOURCE_EXHAUSTED",
            Self::MaintenanceMode { .. } => "MAINTENANCE_MODE",
            Self::FileNotFound { .. } => "FILE_NOT_FOUND",
            Self::FileAccessDenied { .. } => "FILE_ACCESS_DENIED",
            Self::StorageError { .. } => "STORAGE_ERROR",
            Self::RateLimitExceeded { .. } => "RATE_LIMIT_EXCEEDED",
            Self::Unknown { .. } => "UNKNOWN_ERROR",
        }
    }

    /// دریافت پیام خطا برای کاربر
    pub fn user_message(&self) -> String {
        match self {
            Self::AuthenticationFailed { reason, .. } => {
                format!("احراز هویت ناموفق: {}", reason)
            }
            Self::InvalidCredentials { field, message } => {
                format!("اطلاعات نامعتبر در فیلد {}: {}", field, message)
            }
            Self::TokenExpired { .. } => {
                "نشست شما منقضی شده است. لطفاً مجدداً وارد شوید.".to_string()
            }
            Self::TokenInvalid { .. } => {
                "توکن احراز هویت نامعتبر است.".to_string()
            }
            Self::UnauthorizedAccess { required_permission, .. } => {
                format!("شما دسترسی {} را ندارید.", required_permission)
            }
            Self::ValidationError { field, message, .. } => {
                format!("خطا در فیلد {}: {}", field, message)
            }
            Self::InvalidInput { parameter, expected, received } => {
                format!("مقدار نامعتبر برای {}: انتظار {} ولی دریافت {}", parameter, expected, received)
            }
            Self::MissingRequiredField { field } => {
                format!("فیلد {} الزامی است.", field)
            }
            Self::DatabaseError { operation, .. } => {
                format!("خطا در عملیات {}", operation)
            }
            Self::RecordNotFound { entity, id } => {
                format!("{} با شناسه {} یافت نشد.", entity, id)
            }
            Self::DuplicateRecord { entity, field, value } => {
                format!("{} با {} برابر {} قبلاً وجود دارد.", entity, field, value)
            }
            Self::DatabaseConnectionFailed { .. } => {
                "خطا در اتصال به پایگاه داده".to_string()
            }
            Self::NetworkError { .. } => {
                "خطا در ارتباط شبکه".to_string()
            }
            Self::TimeoutError { operation, .. } => {
                format!("عملیات {} زمان زیادی طول کشید.", operation)
            }
            Self::ServiceUnavailable { service, .. } => {
                format!("سرویس {} در دسترس نیست.", service)
            }
            Self::InsufficientFunds { required, available, currency } => {
                format!("موجودی ناکافی: نیاز {} {} ولی {} {} موجود است.", required, currency, available, currency)
            }
            Self::OrderProcessingFailed { order_id, reason } => {
                format!("پردازش سفارش {} ناموفق: {}", order_id, reason)
            }
            Self::PaymentFailed { payment_id, reason, .. } => {
                format!("پرداخت {} ناموفق: {}", payment_id, reason)
            }
            Self::InventoryError { product_id, requested, available } => {
                format!("موجودی ناکافی برای محصول {}: درخواست {} ولی {} موجود است.", product_id, requested, available)
            }
            Self::InternalServerError { .. } => {
                "خطای داخلی سرور. لطفاً بعداً تلاش کنید.".to_string()
            }
            Self::ConfigurationError { parameter, message } => {
                format!("خطا در تنظیمات {}: {}", parameter, message)
            }
            Self::ResourceExhausted { resource, .. } => {
                format!("منبع {} تمام شده است.", resource)
            }
            Self::MaintenanceMode { .. } => {
                "سیستم در حال تعمیر است. لطفاً بعداً تلاش کنید.".to_string()
            }
            Self::FileNotFound { path } => {
                format!("فایل {} یافت نشد.", path)
            }
            Self::FileAccessDenied { path, operation } => {
                format!("دسترسی {} به فایل {} مجاز نیست.", operation, path)
            }
            Self::StorageError { operation, message } => {
                format!("خطا در عملیات ذخیره‌سازی {}: {}", operation, message)
            }
            Self::RateLimitExceeded { retry_after, .. } => {
                format!("تعداد درخواست‌ها زیاد است. {} ثانیه صبر کنید.", retry_after)
            }
            Self::Unknown { message, .. } => {
                format!("خطای نامشخص: {}", message)
            }
        }
    }

    /// تبدیل به JSON برای ارسال به کلاینت
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// ایجاد از JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// بررسی اینکه آیا خطا قابل تلاش مجدد است
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::NetworkError { .. }
                | Self::TimeoutError { .. }
                | Self::ServiceUnavailable { .. }
                | Self::DatabaseConnectionFailed { .. }
                | Self::InternalServerError { .. }
                | Self::ResourceExhausted { .. }
        )
    }

    /// دریافت HTTP status code مناسب
    pub fn http_status_code(&self) -> u16 {
        match self {
            Self::AuthenticationFailed { .. } | Self::InvalidCredentials { .. } => 401,
            Self::TokenExpired { .. } | Self::TokenInvalid { .. } => 401,
            Self::UnauthorizedAccess { .. } => 403,
            Self::ValidationError { .. } | Self::InvalidInput { .. } | Self::MissingRequiredField { .. } => 400,
            Self::RecordNotFound { .. } | Self::FileNotFound { .. } => 404,
            Self::DuplicateRecord { .. } => 409,
            Self::RateLimitExceeded { .. } => 429,
            Self::InternalServerError { .. } | Self::DatabaseError { .. } | Self::ConfigurationError { .. } => 500,
            Self::ServiceUnavailable { .. } | Self::MaintenanceMode { .. } => 503,
            Self::TimeoutError { .. } => 504,
            _ => 500,
        }
    }
}

impl fmt::Display for PemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.user_message())
    }
}

impl std::error::Error for PemaError {}

/// Result type برای عملیات‌های سیستم
pub type PemaResult<T> = Result<T, PemaError>;

/// ماکرو برای ایجاد سریع خطاهای مختلف
#[macro_export]
macro_rules! pema_error {
    (auth_failed, $reason:expr) => {
        PemaError::auth_failed($reason)
    };
    (validation, $field:expr, $message:expr) => {
        PemaError::validation_error($field, $message)
    };
    (database, $operation:expr, $message:expr) => {
        PemaError::database_error($operation, $message)
    };
    (network, $url:expr, $message:expr) => {
        PemaError::network_error($url, $message)
    };
}