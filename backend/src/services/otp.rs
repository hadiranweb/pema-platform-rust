use rand::Rng;
use chrono::{Utc, Duration};
use sqlx::{PgPool, Error};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OtpCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn generate_and_store_otp(pool: &PgPool, user_id: Uuid) -> Result<String, Error> {
    let mut rng = rand::thread_rng();
    let code: u32 = rng.gen_range(100000..999999);
    let code_str = code.to_string();
    let expires_at = Utc::now().naive_utc() + Duration::minutes(5);

    sqlx::query!(
        "INSERT INTO otp_codes (id, user_id, code, expires_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        user_id,
        code_str,
        expires_at
    )
    .execute(pool)
    .await?;

    Ok(code.to_string())
}

pub async fn verify_otp(pool: &PgPool, user_id: Uuid, code: &str) -> Result<bool, Error> {
    let otp_code = sqlx::query_as::<_, OtpCode>(
        "SELECT * FROM otp_codes WHERE user_id = $1 AND code = $2 AND expires_at > NOW()"
    )
    .bind(user_id)
    .bind(code)
    .fetch_optional(pool)
    .await?;

    if otp_code.is_some() {
        // Optionally delete the OTP after successful verification
        sqlx::query!(
            "DELETE FROM otp_codes WHERE user_id = $1 AND code = $2",
            user_id,
            code
        )
        .execute(pool)
        .await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

