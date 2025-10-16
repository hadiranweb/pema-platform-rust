use sqlx::{PgPool, Pool, Postgres};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

/// مدیریت اتصالات دیتابیس با connection pooling پیشرفته
/// این ماژول امکان مدیریت بهینه اتصالات به دیتابیس را فراهم می‌کند

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub ssl_mode: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: std::env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("DATABASE_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .unwrap_or(5432),
            database: std::env::var("DATABASE_NAME").unwrap_or_else(|_| "pema".to_string()),
            username: std::env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string()),
            password: std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "password".to_string()),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .unwrap_or(20),
            min_connections: std::env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            acquire_timeout: std::env::var("DATABASE_ACQUIRE_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            idle_timeout: std::env::var("DATABASE_IDLE_TIMEOUT")
                .unwrap_or_else(|_| "600".to_string())
                .parse()
                .unwrap_or(600),
            max_lifetime: std::env::var("DATABASE_MAX_LIFETIME")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
            ssl_mode: std::env::var("DATABASE_SSL_MODE").unwrap_or_else(|_| "prefer".to_string()),
        }
    }
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}?sslmode={}",
            self.username, self.password, self.host, self.port, self.database, self.ssl_mode
        )
    }

    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self::default())
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pool: PgPool,
    config: DatabaseConfig,
    stats: Arc<RwLock<ConnectionStats>>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ConnectionStats {
    pub total_connections: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
    pub total_queries: u64,
    pub successful_queries: u64,
    pub failed_queries: u64,
    pub average_query_time_ms: f64,
    pub connection_errors: u64,
    pub pool_timeouts: u64,
}

impl DatabaseManager {
    /// ایجاد مدیر دیتابیس جدید
    pub async fn new(config: DatabaseConfig) -> Result<Self, sqlx::Error> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.acquire_timeout))
            .idle_timeout(Duration::from_secs(config.idle_timeout))
            .max_lifetime(Duration::from_secs(config.max_lifetime))
            .connect(&config.connection_string())
            .await?;

        let stats = Arc::new(RwLock::new(ConnectionStats::default()));

        Ok(Self {
            pool,
            config,
            stats,
        })
    }

    /// ایجاد از متغیرهای محیطی
    pub async fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = DatabaseConfig::from_env()?;
        Ok(Self::new(config).await?)
    }

    /// دریافت pool برای استفاده مستقیم
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// بررسی سلامت اتصال
    pub async fn health_check(&self) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                eprintln!("Database health check failed: {}", e);
                Err(e)
            }
        }
    }

    /// دریافت آمار اتصالات
    pub async fn get_stats(&self) -> ConnectionStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// بستن تمام اتصالات
    pub async fn close(&self) {
        self.pool.close().await;
    }

    /// به‌روزرسانی آمار
    async fn update_stats(&self, success: bool, duration_ms: f64) {
        let mut stats = self.stats.write().await;
        stats.total_queries += 1;
        
        if success {
            stats.successful_queries += 1;
        } else {
            stats.failed_queries += 1;
        }

        // محاسبه میانگین زمان query
        let total_successful = stats.successful_queries as f64;
        if total_successful > 0.0 {
            stats.average_query_time_ms = 
                (stats.average_query_time_ms * (total_successful - 1.0) + duration_ms) / total_successful;
        }
    }

    /// دریافت اطلاعات pool
    pub fn pool_info(&self) -> PoolInfo {
        PoolInfo {
            size: self.pool.size(),
            num_idle: self.pool.num_idle(),
            is_closed: self.pool.is_closed(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PoolInfo {
    pub size: u32,
    pub num_idle: u32,
    pub is_closed: bool,
}

/// مدیر چندین دیتابیس
#[derive(Debug)]
pub struct MultiDatabaseManager {
    databases: HashMap<String, DatabaseManager>,
    default_db: String,
}

impl MultiDatabaseManager {
    pub fn new(default_db: String) -> Self {
        Self {
            databases: HashMap::new(),
            default_db,
        }
    }

    /// اضافه کردن دیتابیس جدید
    pub async fn add_database(
        &mut self,
        name: String,
        config: DatabaseConfig,
    ) -> Result<(), sqlx::Error> {
        let manager = DatabaseManager::new(config).await?;
        self.databases.insert(name, manager);
        Ok(())
    }

    /// دریافت مدیر دیتابیس
    pub fn get_database(&self, name: &str) -> Option<&DatabaseManager> {
        self.databases.get(name)
    }

    /// دریافت دیتابیس پیش‌فرض
    pub fn default_database(&self) -> Option<&DatabaseManager> {
        self.databases.get(&self.default_db)
    }

    /// بررسی سلامت تمام دیتابیس‌ها
    pub async fn health_check_all(&self) -> HashMap<String, bool> {
        let mut results = HashMap::new();
        
        for (name, manager) in &self.databases {
            let health = manager.health_check().await.unwrap_or(false);
            results.insert(name.clone(), health);
        }
        
        results
    }

    /// بستن تمام اتصالات
    pub async fn close_all(&self) {
        for manager in self.databases.values() {
            manager.close().await;
        }
    }
}

/// Repository pattern برای عملیات دیتابیس
pub trait Repository<T> {
    async fn find_by_id(&self, id: &str) -> Result<Option<T>, sqlx::Error>;
    async fn find_all(&self) -> Result<Vec<T>, sqlx::Error>;
    async fn create(&self, entity: &T) -> Result<T, sqlx::Error>;
    async fn update(&self, entity: &T) -> Result<T, sqlx::Error>;
    async fn delete(&self, id: &str) -> Result<bool, sqlx::Error>;
}

/// پیاده‌سازی پایه Repository
pub struct BaseRepository<T> {
    db: DatabaseManager,
    table_name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> BaseRepository<T> {
    pub fn new(db: DatabaseManager, table_name: String) -> Self {
        Self {
            db,
            table_name,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn db(&self) -> &DatabaseManager {
        &self.db
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }
}

/// Helper functions
pub mod helpers {
    use super::*;

    /// ایجاد connection string از اجزای جداگانه
    pub fn build_connection_string(
        host: &str,
        port: u16,
        database: &str,
        username: &str,
        password: &str,
        ssl_mode: &str,
    ) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}?sslmode={}",
            username, password, host, port, database, ssl_mode
        )
    }

    /// تست اتصال به دیتابیس
    pub async fn test_connection(connection_string: &str) -> Result<bool, sqlx::Error> {
        let pool = PgPool::connect(connection_string).await?;
        let result = sqlx::query("SELECT 1").fetch_one(&pool).await;
        pool.close().await;
        
        match result {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_config() {
        let config = DatabaseConfig::default();
        assert!(!config.connection_string().is_empty());
        assert!(config.max_connections > 0);
    }

    #[test]
    fn test_connection_string_building() {
        let conn_str = helpers::build_connection_string(
            "localhost",
            5432,
            "test_db",
            "user",
            "pass",
            "require"
        );
        
        assert!(conn_str.contains("postgresql://"));
        assert!(conn_str.contains("localhost:5432"));
        assert!(conn_str.contains("test_db"));
        assert!(conn_str.contains("sslmode=require"));
    }
}