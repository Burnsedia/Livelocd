use crate::auth::TokenValidator;
use once_cell::sync::{Lazy, OnceCell};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

/// In-memory user data: user_id → arbitrary JSON payload
pub static LOCATION_MAP: Lazy<RwLock<HashMap<String, Value>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub static TOKEN_VALIDATOR: OnceCell<Arc<dyn TokenValidator>> = OnceCell::new();

/// Broadcast channel to publish updates to all subscribers
pub static LOCATION_BROADCAST: Lazy<broadcast::Sender<(String, Value)>> = Lazy::new(|| broadcast::channel(1024).0);

