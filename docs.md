# Livelocd Documentation

**Livelocd** is a lightweight, Axum-compatible plugin for real-time location tracking via WebSockets and a JSON API. It's designed to be easily integrated into any Rust backend for applications like live geolocation dashboards, game user tracking, or delivery fleet monitoring.

## Features

- **Real-time Location Updates:** Send and receive location data using WebSockets.
- **Flexible Subscriptions:** Subscribe to location updates for all users or specific users.
- **RESTful API:** Query for the current locations of all users or individual users.
- **Token-Based Authentication:** Secure your WebSocket connections with a flexible token validation system.
- **Minimal Dependencies:** Built with a small footprint, relying on `tokio`, `axum`, and `serde_json`.

## Installation

Add `livelocd` to your `Cargo.toml` dependencies:

```toml
[dependencies]
livelocd = { git = "https://github.com/Burnsedia/livelocd" }
# Or, if published on crates.io:
# livelocd = "0.1.0"
```

## Getting Started

To get started, add the `livelocd` routes to your Axum router.

```rust
use axum::Router;
use livelocd::{livelocd_routes, set_token_validator, ClosureValidator};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // --- 1. Set up your token validator ---
    // For demonstration, we'll allow any token that starts with "valid-".
    // In a real application, you would implement proper token validation.
    let validator = ClosureValidator::new(|token: String| token.starts_with("valid-"));
    set_token_validator(Arc::new(validator));

    // --- 2. Add the livelocd routes to your application ---
    let app = Router::new().merge(livelocd_routes());

    // --- 3. Start your Axum server ---
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
```

## Authentication

Livelocd uses a `TokenValidator` trait to authenticate WebSocket connections. You must set a validator using the `set_token_validator` function before your application starts.

### `TokenValidator` Trait

You can implement the `TokenValidator` trait to create your own custom authentication logic.

```rust
use axum::async_trait;
use livelocd::TokenValidator;

struct MyCustomValidator;

#[async_trait]
impl TokenValidator for MyCustomValidator {
    async fn validate(&self, token: &str) -> bool {
        // Your custom token validation logic here
        // For example, check against a database or an auth service
        token == "secret-token"
    }
}

// Set the validator at startup
// set_token_validator(Arc::new(MyCustomValidator));
```

### `ClosureValidator`

For simpler validation logic, you can use the `ClosureValidator`.

```rust
use livelocd::{set_token_validator, ClosureValidator};
use std::sync::Arc;

let validator = ClosureValidator::new(|token: String| {
    token.starts_with("user-")
});
set_token_validator(Arc::new(validator));
```

## API Reference

### WebSocket API

All WebSocket endpoints require a `token` query parameter for authentication.

- **`GET /ws/send-location?token=<your-token>`**
  
  Send location updates to the server. The server expects JSON messages with a `user_id` and a `location`.

  **Example JSON Payload:**
  ```json
  {
    "user_id": "user123",
    "location": {
      "lat": 33.7489954,
      "lng": -84.3879824
    },
    "status": "online"
  }
  ```

- **`GET /ws/subscribe?token=<your-token>`**

  Subscribe to real-time location updates for all users.

- **`GET /ws/subscribe/:user_id?token=<your-token>`**

  Subscribe to real-time location updates for a specific user.

### REST API

- **`GET /api/users`**

  Get the current known locations for all users.

  **Example Response:**
  ```json
  {
    "user123": {
      "location": { "lat": 33.7489954, "lng": -84.3879824 },
      "status": "online",
      "user_id": "user123"
    }
  }
  ```

- **`GET /api/users/:user_id`**

  Get the most recent location for a single user.

## State Management

Livelocd maintains an in-memory store of user locations.

- **`LOCATION_MAP`**: A `RwLock<HashMap<String, Value>>` that stores the latest JSON payload for each `user_id`.
- **`LOCATION_BROADCAST`**: A `tokio::sync::broadcast` channel that fans out location updates to all subscribers.

## Testing

You can use a command-line tool like `websocat` to test the WebSocket endpoints.

**1. Send a Location:**
```bash
websocat "ws://localhost:3000/ws/send-location?token=valid-token"
# After connecting, paste and send the JSON payload:
{ "user_id": "car-1", "location": { "lat": 40.7, "lng": -74.0 } }
```

**2. Subscribe to All Locations:**
```bash
websocat "ws://localhost:3000/ws/subscribe?token=valid-token"
# You will receive location updates as they are sent.
```

## License

This project is licensed under the MIT License.