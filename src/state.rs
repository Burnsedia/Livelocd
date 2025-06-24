use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::broadcast;
use serde_json::Value;

/// In-memory user data: user_id → arbitrary JSON payload
pub static LOCATION_MAP: Lazy<RwLock<HashMap<String, Value>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

/// Broadcast channel to publish updates to all subscribers
pub static LOCATION_BROADCAST: Lazy<broadcast::Sender<(String, Value)>> = Lazy::new(|| {
    broadcast::channel(1024).0
});

