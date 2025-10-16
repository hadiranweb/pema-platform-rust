use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, mpsc};
use tokio::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::{Result, anyhow};

/// سیستم ثبت‌نام ناهمزمان با اولویت‌بندی
/// این سیستم امکان پردازش درخواست‌های ثبت‌نام را با اولویت‌های مختلف فراهم می‌کند

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RegistrationPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
    Emergency = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub id: Uuid,
    pub user_data: UserRegistrationData,
    pub priority: RegistrationPriority,
    pub created_at: DateTime<Utc>,
    pub retry_count: u32,
    pub max_retries: u32,
    pub timeout: Duration,
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistrationData {
    pub identifier: String, // email or phone
    pub name: String,
    pub password_hash: String,
    pub registration_type: RegistrationType,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationType {
    Email,
    Phone,
    Social { provider: String, external_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationStatus {
    Pending,
    Processing,
    Completed,
    Failed { reason: String },
    Cancelled,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResult {
    pub request_id: Uuid,
    pub status: RegistrationStatus,
    pub user_id: Option<Uuid>,
    pub processed_at: DateTime<Utc>,
    pub processing_time: Duration,
    pub error_message: Option<String>,
}

// Priority queue item for internal use
#[derive(Debug, Clone)]
struct PriorityQueueItem {
    request: RegistrationRequest,
    scheduled_at: Instant,
}

impl PartialEq for PriorityQueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.request.priority == other.request.priority && 
        self.scheduled_at == other.scheduled_at
    }
}

impl Eq for PriorityQueueItem {}

impl PartialOrd for PriorityQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityQueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then earlier scheduled time
        other.request.priority.cmp(&self.request.priority)
            .then_with(|| self.scheduled_at.cmp(&other.scheduled_at))
    }
}

pub struct AsyncRegistrationService {
    // Priority queue for pending requests
    queue: Arc<Mutex<BinaryHeap<PriorityQueueItem>>>,
    
    // Status tracking
    status_map: Arc<RwLock<HashMap<Uuid, RegistrationResult>>>,
    
    // Processing statistics
    stats: Arc<RwLock<RegistrationStats>>,
    
    // Configuration
    config: RegistrationConfig,
    
    // Communication channels
    result_sender: mpsc::UnboundedSender<RegistrationResult>,
    shutdown_sender: mpsc::Sender<()>,
}

#[derive(Debug, Clone)]
pub struct RegistrationConfig {
    pub max_concurrent_workers: usize,
    pub worker_timeout: Duration,
    pub queue_size_limit: usize,
    pub retry_delay: Duration,
    pub cleanup_interval: Duration,
    pub result_retention_time: Duration,
}

impl Default for RegistrationConfig {
    fn default() -> Self {
        Self {
            max_concurrent_workers: 10,
            worker_timeout: Duration::from_secs(30),
            queue_size_limit: 10000,
            retry_delay: Duration::from_secs(5),
            cleanup_interval: Duration::from_secs(300), // 5 minutes
            result_retention_time: Duration::from_secs(3600), // 1 hour
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct RegistrationStats {
    pub total_requests: u64,
    pub completed_requests: u64,
    pub failed_requests: u64,
    pub pending_requests: u64,
    pub average_processing_time: Duration,
    pub queue_size: usize,
    pub active_workers: usize,
}

impl AsyncRegistrationService {
    pub fn new(config: RegistrationConfig) -> Self {
        let (result_sender, _) = mpsc::unbounded_channel();
        let (shutdown_sender, _) = mpsc::channel(1);

        Self {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            status_map: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(RegistrationStats::default())),
            config,
            result_sender,
            shutdown_sender,
        }
    }

    /// شروع سرویس ثبت‌نام ناهمزمان
    pub async fn start(&self) -> Result<()> {
        log::info!("شروع سرویس ثبت‌نام ناهمزمان با {} worker", self.config.max_concurrent_workers);

        // Start worker tasks
        for worker_id in 0..self.config.max_concurrent_workers {
            let service = self.clone();
            tokio::spawn(async move {
                service.worker_loop(worker_id).await;
            });
        }

        // Start cleanup task
        let service = self.clone();
        tokio::spawn(async move {
            service.cleanup_loop().await;
        });

        // Start statistics update task
        let service = self.clone();
        tokio::spawn(async move {
            service.stats_update_loop().await;
        });

        Ok(())
    }

    /// ارسال درخواست ثبت‌نام جدید
    pub async fn submit_registration(&self, mut request: RegistrationRequest) -> Result<Uuid> {
        // Check queue size limit
        {
            let queue = self.queue.lock().await;
            if queue.len() >= self.config.queue_size_limit {
                return Err(anyhow!("صف ثبت‌نام پر است. لطفاً بعداً تلاش کنید."));
            }
        }

        // Generate ID if not provided
        if request.id.is_nil() {
            request.id = Uuid::new_v4();
        }

        // Set default values
        if request.created_at == DateTime::<Utc>::MIN {
            request.created_at = Utc::now();
        }

        // Create initial status
        let initial_result = RegistrationResult {
            request_id: request.id,
            status: RegistrationStatus::Pending,
            user_id: None,
            processed_at: Utc::now(),
            processing_time: Duration::from_secs(0),
            error_message: None,
        };

        // Store status
        {
            let mut status_map = self.status_map.write().await;
            status_map.insert(request.id, initial_result);
        }

        // Add to queue
        let queue_item = PriorityQueueItem {
            request: request.clone(),
            scheduled_at: Instant::now(),
        };

        {
            let mut queue = self.queue.lock().await;
            queue.push(queue_item);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
            stats.pending_requests += 1;
        }

        log::info!("درخواست ثبت‌نام {} با اولویت {:?} به صف اضافه شد", 
                  request.id, request.priority);

        Ok(request.id)
    }

    /// دریافت وضعیت درخواست ثبت‌نام
    pub async fn get_registration_status(&self, request_id: Uuid) -> Option<RegistrationResult> {
        let status_map = self.status_map.read().await;
        status_map.get(&request_id).cloned()
    }

    /// لغو درخواست ثبت‌نام
    pub async fn cancel_registration(&self, request_id: Uuid) -> Result<()> {
        let mut status_map = self.status_map.write().await;
        
        if let Some(result) = status_map.get_mut(&request_id) {
            match result.status {
                RegistrationStatus::Pending | RegistrationStatus::Processing => {
                    result.status = RegistrationStatus::Cancelled;
                    result.processed_at = Utc::now();
                    log::info!("درخواست ثبت‌نام {} لغو شد", request_id);
                    Ok(())
                }
                _ => Err(anyhow!("نمی‌توان درخواست را لغو کرد. وضعیت فعلی: {:?}", result.status))
            }
        } else {
            Err(anyhow!("درخواست ثبت‌نام یافت نشد"))
        }
    }

    /// دریافت آمار سرویس
    pub async fn get_stats(&self) -> RegistrationStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// خاموش کردن سرویس
    pub async fn shutdown(&self) -> Result<()> {
        log::info!("خاموش کردن سرویس ثبت‌نام ناهمزمان...");
        let _ = self.shutdown_sender.send(()).await;
        Ok(())
    }

    // Worker loop for processing registration requests
    async fn worker_loop(&self, worker_id: usize) {
        log::info!("Worker {} شروع شد", worker_id);

        loop {
            // Get next item from queue
            let item = {
                let mut queue = self.queue.lock().await;
                queue.pop()
            };

            if let Some(queue_item) = item {
                let start_time = Instant::now();
                
                // Update status to processing
                {
                    let mut status_map = self.status_map.write().await;
                    if let Some(result) = status_map.get_mut(&queue_item.request.id) {
                        result.status = RegistrationStatus::Processing;
                        result.processed_at = Utc::now();
                    }
                }

                // Process the registration
                let processing_result = self.process_registration(queue_item.request.clone()).await;
                let processing_time = start_time.elapsed();

                // Update final status
                {
                    let mut status_map = self.status_map.write().await;
                    if let Some(result) = status_map.get_mut(&queue_item.request.id) {
                        match processing_result {
                            Ok(user_id) => {
                                result.status = RegistrationStatus::Completed;
                                result.user_id = Some(user_id);
                                
                                // Update stats
                                let mut stats = self.stats.write().await;
                                stats.completed_requests += 1;
                                stats.pending_requests = stats.pending_requests.saturating_sub(1);
                            }
                            Err(e) => {
                                result.status = RegistrationStatus::Failed { 
                                    reason: e.to_string() 
                                };
                                result.error_message = Some(e.to_string());

                                // Retry logic
                                if queue_item.request.retry_count < queue_item.request.max_retries {
                                    let mut retry_request = queue_item.request.clone();
                                    retry_request.retry_count += 1;
                                    
                                    // Schedule retry
                                    let retry_item = PriorityQueueItem {
                                        request: retry_request,
                                        scheduled_at: Instant::now() + self.config.retry_delay,
                                    };

                                    let mut queue = self.queue.lock().await;
                                    queue.push(retry_item);

                                    result.status = RegistrationStatus::Pending;
                                    log::warn!("درخواست {} برای تلاش مجدد برنامه‌ریزی شد (تلاش {}/{})", 
                                              queue_item.request.id, 
                                              queue_item.request.retry_count + 1, 
                                              queue_item.request.max_retries);
                                } else {
                                    // Update stats for final failure
                                    let mut stats = self.stats.write().await;
                                    stats.failed_requests += 1;
                                    stats.pending_requests = stats.pending_requests.saturating_sub(1);
                                }
                            }
                        }
                        
                        result.processing_time = processing_time;
                        result.processed_at = Utc::now();
                    }
                }

                log::debug!("Worker {} درخواست {} را در {:?} پردازش کرد", 
                           worker_id, queue_item.request.id, processing_time);
            } else {
                // No items in queue, wait a bit
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    // Process a single registration request
    async fn process_registration(&self, request: RegistrationRequest) -> Result<Uuid> {
        log::info!("پردازش درخواست ثبت‌نام {} (نوع: {:?})", 
                  request.id, request.user_data.registration_type);

        // Simulate processing time based on priority
        let processing_delay = match request.priority {
            RegistrationPriority::Emergency => Duration::from_millis(100),
            RegistrationPriority::Critical => Duration::from_millis(200),
            RegistrationPriority::High => Duration::from_millis(500),
            RegistrationPriority::Normal => Duration::from_millis(1000),
            RegistrationPriority::Low => Duration::from_millis(2000),
        };

        tokio::time::sleep(processing_delay).await;

        // Validate user data
        self.validate_user_data(&request.user_data).await?;

        // Check for duplicates
        self.check_duplicate_user(&request.user_data).await?;

        // Create user in database
        let user_id = self.create_user_in_database(&request.user_data).await?;

        // Send verification if needed
        match request.user_data.registration_type {
            RegistrationType::Email => {
                self.send_email_verification(&request.user_data.identifier, user_id).await?;
            }
            RegistrationType::Phone => {
                self.send_sms_verification(&request.user_data.identifier, user_id).await?;
            }
            RegistrationType::Social { .. } => {
                // Social registrations are typically pre-verified
            }
        }

        // Call webhook if provided
        if let Some(callback_url) = &request.callback_url {
            let _ = self.call_webhook(callback_url, &request, user_id).await;
        }

        log::info!("ثبت‌نام کاربر {} با موفقیت کامل شد (ID: {})", 
                  request.user_data.identifier, user_id);

        Ok(user_id)
    }

    // Validation methods
    async fn validate_user_data(&self, user_data: &UserRegistrationData) -> Result<()> {
        // Validate identifier format
        match user_data.registration_type {
            RegistrationType::Email => {
                if !user_data.identifier.contains('@') {
                    return Err(anyhow!("فرمت ایمیل نامعتبر است"));
                }
            }
            RegistrationType::Phone => {
                if !user_data.identifier.starts_with('+') || user_data.identifier.len() < 10 {
                    return Err(anyhow!("فرمت شماره تلفن نامعتبر است"));
                }
            }
            RegistrationType::Social { .. } => {
                // Social validation would be done by the provider
            }
        }

        // Validate name
        if user_data.name.trim().is_empty() {
            return Err(anyhow!("نام کاربر نمی‌تواند خالی باشد"));
        }

        // Validate password hash
        if user_data.password_hash.is_empty() {
            return Err(anyhow!("رمز عبور نمی‌تواند خالی باشد"));
        }

        Ok(())
    }

    async fn check_duplicate_user(&self, user_data: &UserRegistrationData) -> Result<()> {
        // This would typically check against the database
        // For now, we'll simulate a check
        
        // Simulate database query delay
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Simulate 5% chance of duplicate
        if rand::random::<f32>() < 0.05 {
            return Err(anyhow!("کاربری با این مشخصات قبلاً ثبت‌نام کرده است"));
        }

        Ok(())
    }

    async fn create_user_in_database(&self, user_data: &UserRegistrationData) -> Result<Uuid> {
        // This would typically insert into the database
        // For now, we'll simulate database insertion
        
        // Simulate database insertion delay
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Simulate 2% chance of database error
        if rand::random::<f32>() < 0.02 {
            return Err(anyhow!("خطا در ذخیره اطلاعات کاربر در پایگاه داده"));
        }

        Ok(Uuid::new_v4())
    }

    async fn send_email_verification(&self, email: &str, user_id: Uuid) -> Result<()> {
        log::info!("ارسال ایمیل تأیید به {} برای کاربر {}", email, user_id);
        
        // Simulate email sending delay
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Simulate 1% chance of email sending failure
        if rand::random::<f32>() < 0.01 {
            return Err(anyhow!("خطا در ارسال ایمیل تأیید"));
        }

        Ok(())
    }

    async fn send_sms_verification(&self, phone: &str, user_id: Uuid) -> Result<()> {
        log::info!("ارسال پیامک تأیید به {} برای کاربر {}", phone, user_id);
        
        // Simulate SMS sending delay
        tokio::time::sleep(Duration::from_millis(300)).await;

        // Simulate 3% chance of SMS sending failure
        if rand::random::<f32>() < 0.03 {
            return Err(anyhow!("خطا در ارسال پیامک تأیید"));
        }

        Ok(())
    }

    async fn call_webhook(&self, url: &str, request: &RegistrationRequest, user_id: Uuid) -> Result<()> {
        log::info!("فراخوانی webhook {} برای درخواست {}", url, request.id);
        
        // This would typically make an HTTP request to the webhook URL
        // For now, we'll simulate it
        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(())
    }

    // Cleanup loop to remove old results
    async fn cleanup_loop(&self) {
        let mut interval = tokio::time::interval(self.config.cleanup_interval);
        
        loop {
            interval.tick().await;
            
            let cutoff_time = Utc::now() - chrono::Duration::from_std(self.config.result_retention_time).unwrap();
            let mut removed_count = 0;

            {
                let mut status_map = self.status_map.write().await;
                status_map.retain(|_, result| {
                    let should_keep = result.processed_at > cutoff_time;
                    if !should_keep {
                        removed_count += 1;
                    }
                    should_keep
                });
            }

            if removed_count > 0 {
                log::info!("پاک‌سازی: {} نتیجه قدیمی حذف شد", removed_count);
            }
        }
    }

    // Statistics update loop
    async fn stats_update_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        
        loop {
            interval.tick().await;
            
            let queue_size = {
                let queue = self.queue.lock().await;
                queue.len()
            };

            {
                let mut stats = self.stats.write().await;
                stats.queue_size = queue_size;
                // Other stats are updated in real-time during processing
            }
        }
    }
}

impl Clone for AsyncRegistrationService {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
            status_map: Arc::clone(&self.status_map),
            stats: Arc::clone(&self.stats),
            config: self.config.clone(),
            result_sender: self.result_sender.clone(),
            shutdown_sender: self.shutdown_sender.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registration_service() {
        let config = RegistrationConfig::default();
        let service = AsyncRegistrationService::new(config);
        
        // Start the service
        service.start().await.unwrap();

        // Create a test registration request
        let request = RegistrationRequest {
            id: Uuid::new_v4(),
            user_data: UserRegistrationData {
                identifier: "test@example.com".to_string(),
                name: "Test User".to_string(),
                password_hash: "hashed_password".to_string(),
                registration_type: RegistrationType::Email,
                metadata: HashMap::new(),
            },
            priority: RegistrationPriority::Normal,
            created_at: Utc::now(),
            retry_count: 0,
            max_retries: 3,
            timeout: Duration::from_secs(30),
            callback_url: None,
        };

        // Submit the request
        let request_id = service.submit_registration(request).await.unwrap();

        // Wait a bit for processing
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Check the status
        let status = service.get_registration_status(request_id).await;
        assert!(status.is_some());

        let stats = service.get_stats().await;
        assert!(stats.total_requests > 0);
    }
}