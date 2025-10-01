use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use models::payment::{Payment, CreatePayment, PaymentStatus};
use crate::error::ServiceError;

pub async fn process_payment(pool: &PgPool, new_payment: CreatePayment) -> Result<Payment, ServiceError> {
    // Simulate payment processing
    let transaction_id = Uuid::new_v4().to_string();
    let payment_status = "completed".to_string(); // In a real scenario, this would come from a payment gateway

    let payment = sqlx::query_as::<_, Payment>(
        "INSERT INTO payments (order_id, amount, status, transaction_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(new_payment.order_id)
    .bind(new_payment.amount)
    .bind(payment_status)
    .bind(transaction_id)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(payment)
}

pub async fn get_payment_status(pool: &PgPool, payment_id: Uuid) -> Result<Payment, ServiceError> {
    let payment = sqlx::query_as::<_, Payment>(
        "SELECT * FROM payments WHERE id = $1"
    )
    .bind(payment_id)
    .fetch_one(pool)
    .await?;

    Ok(payment)
}

