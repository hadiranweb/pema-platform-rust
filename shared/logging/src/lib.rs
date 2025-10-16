use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// سیستم لاگینگ پیشرفته برای پلتفرم PEMA
/// این ماژول امکان ثبت، ذخیره و تحلیل لاگ‌ها را فراهم می‌کند

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRACE" => Some(LogLevel::Trace),
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" => Some(LogLevel::Warn),
            "ERROR" => Some(LogLevel::Error),
            "FATAL" => Some(LogLevel::Fatal),
            _ => None,
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            LogLevel::Trace => "\x1b[37m",    // White
            LogLevel::Debug => "\x1b[36m",    // Cyan
            LogLevel::Info => "\x1b[32m",     // Green
            LogLevel::Warn => "\x1b[33m",     // Yellow
            LogLevel::Error => "\x1b[31m",    // Red
            LogLevel::Fatal => "\x1b[35m",    // Magenta
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub module: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub target: String,
    pub thread_id: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub request_id: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub tags: Vec<String>,
    pub duration_ms: Option<u64>,
    pub error_code: Option<String>,
    pub stack_trace: Option<String>,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: String, module: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            level,
            message,
            module,
            file: None,
            line: None,
            target: "pema".to_string(),
            thread_id: format!("{:?}", std::thread::current().id()),
            user_id: None,
            session_id: None,
            request_id: None,
            metadata: HashMap::new(),
            tags: Vec::new(),
            duration_ms: None,
            error_code: None,
            stack_trace: None,
        }
    }

    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_request(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    pub fn with_error_code(mut self, error_code: String) -> Self {
        self.error_code = Some(error_code);
        self
    }

    pub fn with_stack_trace(mut self, stack_trace: String) -> Self {
        self.stack_trace = Some(stack_trace);
        self
    }

    pub fn formatted(&self, colored: bool) -> String {
        let color_start = if colored { self.level.color_code() } else { "" };
        let color_end = if colored { "\x1b[0m" } else { "" };

        let mut formatted = format!(
            "{}{} [{}] {} - {}{}",
            color_start,
            self.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
            self.level.as_str(),
            self.module,
            self.message,
            color_end
        );

        if let Some(user_id) = &self.user_id {
            formatted.push_str(&format!(" [user:{}]", user_id));
        }

        if let Some(request_id) = &self.request_id {
            formatted.push_str(&format!(" [req:{}]", request_id));
        }

        if let Some(duration) = self.duration_ms {
            formatted.push_str(&format!(" [{}ms]", duration));
        }

        if !self.tags.is_empty() {
            formatted.push_str(&format!(" [tags:{}]", self.tags.join(",")));
        }

        formatted
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// تنظیمات سیستم لاگینگ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub min_level: LogLevel,
    pub enable_console: bool,
    pub enable_file: bool,
    pub file_path: Option<String>,
    pub max_file_size_mb: u64,
    pub max_files: u32,
    pub buffer_size: usize,
    pub flush_interval_ms: u64,
    pub colored_output: bool,
    pub include_location: bool,
    pub include_thread_id: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            min_level: LogLevel::Info,
            enable_console: true,
            enable_file: false,
            file_path: Some("logs/pema.log".to_string()),
            max_file_size_mb: 100,
            max_files: 10,
            buffer_size: 1000,
            flush_interval_ms: 5000,
            colored_output: true,
            include_location: true,
            include_thread_id: true,
        }
    }
}

/// مدیر اصلی سیستم لاگینگ
pub struct Logger {
    config: LogConfig,
    buffer: Arc<RwLock<Vec<LogEntry>>>,
    stats: Arc<RwLock<LogStats>>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct LogStats {
    pub total_logs: u64,
    pub logs_by_level: HashMap<String, u64>,
    pub logs_by_module: HashMap<String, u64>,
    pub errors_count: u64,
    pub warnings_count: u64,
    pub last_error: Option<DateTime<Utc>>,
    pub last_warning: Option<DateTime<Utc>>,
    pub average_logs_per_minute: f64,
    pub start_time: DateTime<Utc>,
}

impl Logger {
    pub fn new(config: LogConfig) -> Self {
        Self {
            config,
            buffer: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(LogStats {
                start_time: Utc::now(),
                ..Default::default()
            })),
        }
    }

    pub async fn log(&self, entry: LogEntry) {
        // بررسی سطح minimum
        if entry.level < self.config.min_level {
            return;
        }

        // به‌روزرسانی آمار
        self.update_stats(&entry).await;

        // نمایش در console
        if self.config.enable_console {
            println!("{}", entry.formatted(self.config.colored_output));
        }

        // ذخیره در فایل
        if self.config.enable_file {
            if let Some(file_path) = &self.config.file_path {
                self.write_to_file(&entry, file_path).await;
            }
        }

        // اضافه کردن به buffer
        {
            let mut buffer = self.buffer.write().await;
            buffer.push(entry);

            // flush کردن buffer در صورت نیاز
            if buffer.len() >= self.config.buffer_size {
                buffer.clear();
            }
        }
    }

    pub async fn trace(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Trace, message.to_string(), module.to_string());
        self.log(entry).await;
    }

    pub async fn debug(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Debug, message.to_string(), module.to_string());
        self.log(entry).await;
    }

    pub async fn info(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Info, message.to_string(), module.to_string());
        self.log(entry).await;
    }

    pub async fn warn(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Warn, message.to_string(), module.to_string());
        self.log(entry).await;
    }

    pub async fn error(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Error, message.to_string(), module.to_string());
        self.log(entry).await;
    }

    pub async fn fatal(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Fatal, message.to_string(), module.to_string());
        self.log(entry).await;
    }

    pub async fn get_stats(&self) -> LogStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    pub async fn flush(&self) {
        let mut buffer = self.buffer.write().await;
        buffer.clear();
    }

    async fn update_stats(&self, entry: &LogEntry) {
        let mut stats = self.stats.write().await;
        stats.total_logs += 1;

        // آمار بر اساس سطح
        let level_key = entry.level.as_str().to_string();
        *stats.logs_by_level.entry(level_key).or_insert(0) += 1;

        // آمار بر اساس ماژول
        *stats.logs_by_module.entry(entry.module.clone()).or_insert(0) += 1;

        // آمار خطاها و هشدارها
        match entry.level {
            LogLevel::Error | LogLevel::Fatal => {
                stats.errors_count += 1;
                stats.last_error = Some(entry.timestamp);
            }
            LogLevel::Warn => {
                stats.warnings_count += 1;
                stats.last_warning = Some(entry.timestamp);
            }
            _ => {}
        }

        // محاسبه میانگین لاگ در دقیقه
        let duration = Utc::now().signed_duration_since(stats.start_time);
        let minutes = duration.num_minutes() as f64;
        if minutes > 0.0 {
            stats.average_logs_per_minute = stats.total_logs as f64 / minutes;
        }
    }

    async fn write_to_file(&self, entry: &LogEntry, file_path: &str) {
        if let Ok(mut file) = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .await
        {
            let log_line = format!("{}\n", entry.formatted(false));
            let _ = tokio::io::AsyncWriteExt::write_all(&mut file, log_line.as_bytes()).await;
        }
    }
}

/// ماکروهای راحت برای لاگینگ
#[macro_export]
macro_rules! log_trace {
    ($logger:expr, $msg:expr) => {
        $logger.trace($msg, module_path!()).await
    };
    ($logger:expr, $msg:expr, $($arg:tt)*) => {
        $logger.trace(&format!($msg, $($arg)*), module_path!()).await
    };
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $msg:expr) => {
        $logger.debug($msg, module_path!()).await
    };
    ($logger:expr, $msg:expr, $($arg:tt)*) => {
        $logger.debug(&format!($msg, $($arg)*), module_path!()).await
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $msg:expr) => {
        $logger.info($msg, module_path!()).await
    };
    ($logger:expr, $msg:expr, $($arg:tt)*) => {
        $logger.info(&format!($msg, $($arg)*), module_path!()).await
    };
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $msg:expr) => {
        $logger.warn($msg, module_path!()).await
    };
    ($logger:expr, $msg:expr, $($arg:tt)*) => {
        $logger.warn(&format!($msg, $($arg)*), module_path!()).await
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $msg:expr) => {
        $logger.error($msg, module_path!()).await
    };
    ($logger:expr, $msg:expr, $($arg:tt)*) => {
        $logger.error(&format!($msg, $($arg)*), module_path!()).await
    };
}

/// Helper functions
pub mod helpers {
    use super::*;

    /// ایجاد request ID یکتا
    pub fn generate_request_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// تبدیل error به log entry
    pub fn error_to_log_entry(error: &dyn std::error::Error, module: &str) -> LogEntry {
        LogEntry::new(
            LogLevel::Error,
            error.to_string(),
            module.to_string(),
        )
        .with_error_code("UNKNOWN_ERROR".to_string())
    }

    /// اندازه‌گیری زمان اجرای عملیات
    pub struct Timer {
        start: std::time::Instant,
        name: String,
        module: String,
    }

    impl Timer {
        pub fn new(name: String, module: String) -> Self {
            Self {
                start: std::time::Instant::now(),
                name,
                module,
            }
        }

        pub async fn finish(self, logger: &Logger) {
            let duration = self.start.elapsed();
            let entry = LogEntry::new(
                LogLevel::Debug,
                format!("Operation '{}' completed", self.name),
                self.module,
            )
            .with_duration(duration.as_millis() as u64)
            .with_tag("performance".to_string());

            logger.log(entry).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
        assert!(LogLevel::Error < LogLevel::Fatal);
    }

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new(
            LogLevel::Info,
            "Test message".to_string(),
            "test_module".to_string(),
        );

        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "Test message");
        assert_eq!(entry.module, "test_module");
        assert!(!entry.id.is_empty());
    }

    #[tokio::test]
    async fn test_logger_basic_functionality() {
        let config = LogConfig::default();
        let logger = Logger::new(config);

        logger.info("Test log message", "test").await;

        let stats = logger.get_stats().await;
        assert_eq!(stats.total_logs, 1);
    }
}