use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

pub mod bus;

pub use bus::EventBus;

/// Event system for the platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub event_type: EventType,
    pub tenant_id: String,
    pub user_id: Option<String>,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: EventSource,
    pub correlation_id: Option<String>,
}

/// Event types in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventType {
    // User events
    UserRegistered,
    UserLoggedIn,
    UserLoggedOut,
    UserUpdated,
    
    // Order events
    OrderCreated,
    OrderUpdated,
    OrderCancelled,
    OrderCompleted,
    
    // Payment events
    PaymentProcessed,
    PaymentFailed,
    PaymentRefunded,
    
    // Product events
    ProductViewed,
    ProductAdded,
    ProductUpdated,
    ProductRemoved,
    
    // Cart events
    CartUpdated,
    CartAbandoned,
    
    // Plugin events
    PluginLoaded,
    PluginUnloaded,
    PluginExecuted,
    PluginError,
    
    // System events
    SystemStarted,
    SystemShutdown,
    HealthCheck,
    
    // Custom events (for plugins)
    Custom(String),
}

/// Event source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSource {
    System,
    User,
    Plugin(String),
    External(String),
}

/// Event handler trait
pub trait EventHandler: Send + Sync {
    fn handle_event(&self, event: &Event) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn event_types(&self) -> Vec<EventType>;
}

impl Event {
    pub fn new(
        event_type: EventType,
        tenant_id: String,
        data: serde_json::Value,
        source: EventSource,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_type,
            tenant_id,
            user_id: None,
            data,
            timestamp: chrono::Utc::now(),
            source,
            correlation_id: None,
        }
    }

    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: String) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
}