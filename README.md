# Livelocd

**Livelocd** is a lightweight Axum-compatible plugin for real-time location tracking via WebSockets and a JSON API. Easily drop it into any Rust backend to enable live geolocation dashboards, game user tracking, or delivery fleet monitoring.

---

## ✨ Features

- 📡 WebSocket support for sending real-time location updates  
- 👂 Subscribe to all users or individual users' locations  
- 🌐 REST API to query current locations  
- 🧩 Designed as a plugin for Axum or Loco.rs apps  
- ⚡ Built with minimal dependencies, powered by `tokio`, `axum`, and `serde_json`  

---

## 📦 Installation

In your project’s `Cargo.toml`:

```toml
livelocd = { path = "../livelocd" } # or use Git/crates.io in the future
```

---

## 🔌 Usage

In your Axum project:

```rust
use axum::Router;
use livelocd::livelocd_routes;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(livelocd_routes());
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

## 📡 WebSocket Endpoints

- `GET /ws/send-location` — Send JSON with a `user_id` and any arbitrary fields (e.g., lat/lng)  
- `GET /ws/subscribe` — Receive real-time updates for all users  
- `GET /ws/subscribe/:user_id` — Subscribe to updates for a specific user  

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

## 🔒 Privacy & Security

You are responsible for securing the WebSocket and API endpoints (auth, rate limiting, etc.) based on your use case.

---

## 🚀 Use Cases

- Live fleet or delivery tracking  
- Multiplayer game player positions  
- Dashboards for location-aware apps  
- IoT geolocation feeds  

---

## 🧪 Testing Locally

Use [`websocat`](https://github.com/vi/websocat):

```bash
# Send location
websocat ws://localhost:3000/ws/send-location
{"user_id": "car-1", "lat": 40.7, "lng": -74.0}

# Subscribe to all
websocat ws://localhost:3000/ws/subscribe
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

