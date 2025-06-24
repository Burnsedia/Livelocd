//! # Livelocd
//!
//! **Livelocd** is a lightweight Axum-compatible plugin for real-time location tracking using WebSockets.
//!
//! ## Features
//!
//! - Send location updates via WebSocket
//! - Broadcast to all or specific subscribers
//! - Query all users or specific ones via REST
//!
//! ## Usage
//! ```rust
//! use axum::{Router};
//! use livelocd::livelocd_routes;
//!
//! let app = Router::new().merge(livelocd_routes());
//! ```
//!
//! ## WebSocket Endpoints
//!
//! - `GET /ws/send-location`: Send JSON like `{"user_id": "user123", "location": "33.75,-84.38"}`
//! - `GET /ws/subscribe`: Subscribe to all location updates
//! - `GET /ws/subscribe/:user_id`: Subscribe to a single user’s updates
//!
//! ## REST API Endpoints
//!
//! - `GET /api/users`: Get all tracked user locations
//! - `GET /api/users/:user_id`: Get location for a single user
//!
//! ## License
//! MIT © [Bailey Burnsed](https://github.com/Burnsedia)

mod handlers;
mod state;

use axum::{routing::get, Router};
use handlers::*;

pub fn routes() -> Router {
    Router::new()
        .route("/ws/send-location", get(send_location))
        .route("/ws/subscribe", get(subscribe_all))
        .route("/ws/subscribe/:user_id", get(subscribe_one))
        .route("/api/users", get(get_all_users))
        .route("/api/users/:user_id", get(get_user))
}
