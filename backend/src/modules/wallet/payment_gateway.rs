use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PaymentGateway {
    async fn process_payment(&self, user_id: Uuid, amount: f64) -> Result<String, String>;
    async fn refund_payment(&self, user_id: Uuid, amount: f64) -> Result<String, String>;
}

pub struct MockPaymentGateway;

#[async_trait]
impl PaymentGateway for MockPaymentGateway {
    async fn process_payment(&self, user_id: Uuid, amount: f64) -> Result<String, String> {
        println!("MockPaymentGateway: Processing payment for user {} with amount {}", user_id, amount);
        Ok(format!("mock_payment_id_{}", Uuid::new_v4()))
    }

    async fn refund_payment(&self, user_id: Uuid, amount: f64) -> Result<String, String> {
        println!("MockPaymentGateway: Refunding payment for user {} with amount {}", user_id, amount);
        Ok(format!("mock_refund_id_{}", Uuid::new_v4()))
    }
}

