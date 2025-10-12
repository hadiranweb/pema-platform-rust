use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
}

pub enum PluginHookType {
    OnOrderCreated,
    CalculateDiscount,
    // Add other hook types as needed
}

pub trait PluginInterface {
    fn get_metadata() -> PluginMetadata;
    // Define common plugin functions here
    // fn on_order_created(order: Order) -> Result<(), String>;
}

