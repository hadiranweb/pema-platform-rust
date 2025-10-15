
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use models::user::{User, UserRegister};

pub async fn create_user(pool: &PgPool, user_register: UserRegister, password_hash: String) -> Result<User, String> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(user_register.username)
    .bind(user_register.email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to create user: {}", e))?;

    Ok(user)
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<User, String> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to fetch user by email: {}", e))?
    .ok_or_else(|| "User not found".to_string())?;

    Ok(user)
}

