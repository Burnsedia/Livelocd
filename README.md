# Livelocd

**Livelocd** is a lightweight Axum-compatible plugin for real-time location tracking via WebSockets and a JSON API. Easily drop it into any Rust backend to enable live geolocation dashboards, game user tracking, or delivery fleet monitoring.

---

## ✨ Features

- 📡 WebSocket support for sending real-time location updates
- 👂 Subscribe to all users or individual users' locations
- 🌐 REST API to query current locations
- 🔐 **Token-based authentication** for WebSocket connections
- 🧩 Designed as a plugin for Axum apps
- ⚡ Built with minimal dependencies, powered by `tokio`, `axum`, and `serde_json`

---

## 📚 For detailed documentation, please see [docs.md](docs.md).

---

## 📦 Installation

In your project’s `Cargo.toml`:

```toml
[dependencies]
livelocd = { git = "https://github.com/Burnsedia/livelocd" }
```

---

## 🔌 Usage

In your Axum project:

```rust
use axum::Router;
use livelocd::{livelocd_routes, set_token_validator, ClosureValidator};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 1. Set up your token validator
    let validator = ClosureValidator::new(|token: String| token.starts_with("valid-"));
    set_token_validator(Arc::new(validator));

    // 2. Add the livelocd routes
    let app = Router::new().merge(livelocd_routes());

    // 3. Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
```

---

## 📡 WebSocket Endpoints

All WebSocket endpoints require a `token` query parameter.

- `GET /ws/send-location?token=<your-token>` — Send JSON with a `user_id` and any arbitrary fields (e.g., lat/lng)
- `GET /ws/subscribe?token=<your-token>` — Receive real-time updates for all users
- `GET /ws/subscribe/:user_id?token=<your-token>` — Subscribe to updates for a specific user

### Example JSON payload:

```json
{
  "user_id": "user123",
  "lat": 33.7489954,
  "lng": -84.3879824,
  "status": "moving"
}
```

---

## 🔍 REST API Endpoints

- `GET /api/users` — Get current known location for all users
- `GET /api/users/:user_id` — Get most recent location for a single user

---

## 🧪 Testing Locally

Use [`websocat`](https://github.com/vi/websocat):

```bash
# Send location
websocat "ws://localhost:3000/ws/send-location?token=valid-token"
# Then, send the JSON payload:
{"user_id": "car-1", "lat": 40.7, "lng": -74.0}

# Subscribe to all
websocat "ws://localhost:3000/ws/subscribe?token=valid-token"
```

---

## 🛠 Built With

- [Axum](https://github.com/tokio-rs/axum)
- [Serde](https://serde.rs/)
- [Tokio](https://tokio.rs/)

---

## 📄 License

MIT

---

## 🤝 Contributing

Pull requests welcome! Let’s make real-time dashboards in Rust even easier.


[Made with Love by Bailey Burnsed](https://baileyburnsed.dev/)