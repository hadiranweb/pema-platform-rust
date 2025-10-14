
// This module will define the state machine for order processing.
// It will handle transitions between order statuses (e.g., pending, processing, shipped, delivered, cancelled).
// This will ensure that order status changes follow a defined workflow and prevent invalid transitions.

#[derive(Debug)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    pub fn can_transition_to(&self, new_status: &OrderStatus) -> bool {
        match self {
            OrderStatus::Pending => matches!(new_status, OrderStatus::Processing | OrderStatus::Cancelled),
            OrderStatus::Processing => matches!(new_status, OrderStatus::Shipped | OrderStatus::Cancelled),
            OrderStatus::Shipped => matches!(new_status, OrderStatus::Delivered | OrderStatus::Cancelled),
            OrderStatus::Delivered => false, // Cannot transition from delivered
            OrderStatus::Cancelled => false, // Cannot transition from cancelled
        }
    }
}

// Placeholder function to apply a status transition
pub fn apply_transition(current_status: OrderStatus, new_status: OrderStatus) -> Result<OrderStatus, String> {
    if current_status.can_transition_to(&new_status) {
        Ok(new_status)
    } else {
        Err(format!("Invalid status transition from {:?} to {:?}", current_status, new_status))
    }
}

