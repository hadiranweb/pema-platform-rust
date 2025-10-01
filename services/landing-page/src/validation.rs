use regex::Regex;
use crate::error::{AppError, AppResult};

pub struct Validator;

impl Validator {
    pub fn validate_email(email: &str) -> AppResult<()> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| AppError::ValidationError("خطا در بررسی ایمیل".to_string()))?;
        
        if email_regex.is_match(email) {
            Ok(())
        } else {
            Err(AppError::ValidationError("فرمت ایمیل نادرست است".to_string()))
        }
    }

    pub fn validate_phone(phone: &str) -> AppResult<()> {
        let phone_regex = Regex::new(r"^(\+98|0)?9\d{9}$")
            .map_err(|_| AppError::ValidationError("خطا در بررسی شماره تلفن".to_string()))?;
        
        if phone_regex.is_match(phone) {
            Ok(())
        } else {
            Err(AppError::ValidationError("فرمت شماره تلفن نادرست است".to_string()))
        }
    }

    pub fn validate_username(username: &str) -> AppResult<()> {
        if username.len() < 3 {
            return Err(AppError::ValidationError("نام کاربری باید حداقل ۳ کاراکتر باشد".to_string()));
        }
        
        if username.len() > 50 {
            return Err(AppError::ValidationError("نام کاربری نباید بیش از ۵۰ کاراکتر باشد".to_string()));
        }

        let username_regex = Regex::new(r"^[a-zA-Z0-9_]+$")
            .map_err(|_| AppError::ValidationError("خطا در بررسی نام کاربری".to_string()))?;
        
        if username_regex.is_match(username) {
            Ok(())
        } else {
            Err(AppError::ValidationError("نام کاربری فقط می‌تواند شامل حروف انگلیسی، اعداد و _ باشد".to_string()))
        }
    }

    pub fn validate_password(password: &str) -> AppResult<()> {
        if password.len() < 8 {
            return Err(AppError::ValidationError("رمز عبور باید حداقل ۸ کاراکتر باشد".to_string()));
        }
        
        if password.len() > 128 {
            return Err(AppError::ValidationError("رمز عبور نباید بیش از ۱۲۸ کاراکتر باشد".to_string()));
        }

        // Check for at least one uppercase letter
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(AppError::ValidationError("رمز عبور باید حداقل یک حرف بزرگ داشته باشد".to_string()));
        }

        // Check for at least one lowercase letter
        if !password.chars().any(|c| c.is_lowercase()) {
            return Err(AppError::ValidationError("رمز عبور باید حداقل یک حرف کوچک داشته باشد".to_string()));
        }

        // Check for at least one digit
        if !password.chars().any(|c| c.is_numeric()) {
            return Err(AppError::ValidationError("رمز عبور باید حداقل یک عدد داشته باشد".to_string()));
        }

        Ok(())
    }

    pub fn validate_verification_code(code: &str) -> AppResult<()> {
        if code.len() != 4 {
            return Err(AppError::ValidationError("کد تایید باید ۴ رقم باشد".to_string()));
        }

        if !code.chars().all(|c| c.is_numeric()) {
            return Err(AppError::ValidationError("کد تایید فقط باید شامل اعداد باشد".to_string()));
        }

        Ok(())
    }

    pub fn validate_email_or_phone(input: &str) -> AppResult<EmailOrPhone> {
        if input.contains('@') {
            Self::validate_email(input)?;
            Ok(EmailOrPhone::Email(input.to_string()))
        } else {
            Self::validate_phone(input)?;
            Ok(EmailOrPhone::Phone(input.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub enum EmailOrPhone {
    Email(String),
    Phone(String),
}

impl EmailOrPhone {
    pub fn as_str(&self) -> &str {
        match self {
            EmailOrPhone::Email(email) => email,
            EmailOrPhone::Phone(phone) => phone,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(Validator::validate_email("test@example.com").is_ok());
        assert!(Validator::validate_email("user.name+tag@domain.co.uk").is_ok());
        assert!(Validator::validate_email("invalid-email").is_err());
        assert!(Validator::validate_email("@domain.com").is_err());
    }

    #[test]
    fn test_validate_phone() {
        assert!(Validator::validate_phone("09123456789").is_ok());
        assert!(Validator::validate_phone("+989123456789").is_ok());
        assert!(Validator::validate_phone("9123456789").is_ok());
        assert!(Validator::validate_phone("0812345678").is_err());
        assert!(Validator::validate_phone("091234567890").is_err());
    }

    #[test]
    fn test_validate_username() {
        assert!(Validator::validate_username("user123").is_ok());
        assert!(Validator::validate_username("test_user").is_ok());
        assert!(Validator::validate_username("ab").is_err());
        assert!(Validator::validate_username("user-name").is_err());
        assert!(Validator::validate_username("user name").is_err());
    }

    #[test]
    fn test_validate_password() {
        assert!(Validator::validate_password("Password123").is_ok());
        assert!(Validator::validate_password("MySecure1").is_ok());
        assert!(Validator::validate_password("password").is_err()); // no uppercase
        assert!(Validator::validate_password("PASSWORD123").is_err()); // no lowercase
        assert!(Validator::validate_password("Password").is_err()); // no digit
        assert!(Validator::validate_password("Pass1").is_err()); // too short
    }

    #[test]
    fn test_validate_verification_code() {
        assert!(Validator::validate_verification_code("1234").is_ok());
        assert!(Validator::validate_verification_code("0000").is_ok());
        assert!(Validator::validate_verification_code("123").is_err()); // too short
        assert!(Validator::validate_verification_code("12345").is_err()); // too long
        assert!(Validator::validate_verification_code("12a4").is_err()); // contains letter
    }
}
