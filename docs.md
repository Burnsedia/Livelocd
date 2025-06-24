# Livelocd Documentation

**Livelocd** is a lightweight plugin for [Axum](https://github.com/tokio-rs/axum) that enables **real-time location tracking** using WebSockets.

## 🔧 Installation

```toml
# Cargo.toml
[dependencies]
livelocd = "0.1.0"
```

## 🚀 Features

- Realtime WebSocket tracking for any client
- Broadcast to all clients or to individual subscribers
- JSON API to fetch user location(s)
- Built on Axum and Tokio — fast and async-native
- Easily embeddable in other projects

---

## 📦 Usage

```rust
use axum::{Router};
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

### `GET /ws/send-location`

Send JSON objects from clients in the following format:

```json
{
  "user_id": "user123",
  "location": "33.7489954,-84.3879824"
}
```

### `GET /ws/subscribe`

Broadcasts **all user locations** to subscribers in real-time.

### `GET /ws/subscribe/:user_id`

Broadcasts updates only for a **specific user**.

---

## 📊 REST API Endpoints

### `GET /api/users`

Returns a JSON map of all users and their last known locations:

```json
{
  "user123": "33.7489954,-84.3879824",
  "user456": "40.712776,-74.005974"
}
```

### `GET /api/users/:user_id`

Returns a single user’s location:

```json
{
  "user_id": "user123",
  "location": "33.7489954,-84.3879824"
}
```

---

## 🧩 Integration Ideas

- Live delivery driver tracking
- Real-time multiplayer player positions
- IoT device geolocation
- Emergency response dashboards

---

## ⚖ License

MIT © [Bailey Burnsed](https://github.com/Burnsedia)

## 🔗 Repository

[https://github.com/Burnsedia/livelocd](https://github.com/Burnsedia/livelocd)

