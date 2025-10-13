use super::*;
use tokio::sync::broadcast;
use std::collections::HashMap;
use std::sync::Arc;

/// Event bus for publishing and subscribing to events
pub struct EventBus {
    sender: broadcast::Sender<Event>,
    handlers: Arc<RwLock<HashMap<EventType, Vec<Arc<dyn EventHandler>>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1000);
        
        Self {
            sender,
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Publish an event
    pub async fn publish(&self, event: Event) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Send to broadcast channel
        let _ = self.sender.send(event.clone());

        // Execute registered handlers
        let handlers = self.handlers.read().await;
        if let Some(event_handlers) = handlers.get(&event.event_type) {
            for handler in event_handlers {
                if let Err(e) = handler.handle_event(&event) {
                    tracing::error!("Event handler error: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    /// Register an event handler
    pub async fn register_handler(&self, handler: Arc<dyn EventHandler>) {
        let mut handlers = self.handlers.write().await;
        
        for event_type in handler.event_types() {
            handlers
                .entry(event_type)
                .or_insert_with(Vec::new)
                .push(handler.clone());
        }
    }
}

// Note: This is a simplified implementation. In production, you'd want:
// 1. Proper error handling for handler registration
// 2. Handler removal functionality  
// 3. Event persistence for reliability
// 4. Dead letter queue for failed events
// 5. Event replay capabilities