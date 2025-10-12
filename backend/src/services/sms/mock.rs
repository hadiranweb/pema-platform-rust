use async_trait::async_trait;

#[async_trait]
pub trait SmsService {
    async fn send_sms(&self, to: &str, message: &str) -> Result<(), String>;
}

pub struct MockSmsService;

#[async_trait]
impl SmsService for MockSmsService {
    async fn send_sms(&self, to: &str, message: &str) -> Result<(), String> {
        println!("Mock SMS Service: Sending SMS to {} with message: {}", to, message);
        Ok(())
    }
}

