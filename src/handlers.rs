use axum::{
    extract::{ws::{WebSocketUpgrade, Message}, Path},
    response::IntoResponse,
    Json,
};
use futures::StreamExt;
use serde_json::Value;

use crate::state::{LOCATION_MAP, LOCATION_BROADCAST};

/// Accepts WebSocket location updates as JSON
/// Expected format: `{ "user_id": "user123", "location": "33.7,-84.3" }`
pub async fn send_location(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        while let Some(Ok(Message::Text(text))) = socket.next().await {
            match serde_json::from_str::<Value>(&text) {
                Ok(json) => {
                    if let Some(user_id) = json.get("user_id").and_then(|v| v.as_str()) {
                        LOCATION_MAP.write().unwrap().insert(user_id.to_string(), json.clone());
                        let _ = LOCATION_BROADCAST.send((user_id.to_string(), json));
                    }
                }
                Err(err) => {
                    eprintln!("❌ Invalid JSON: {}", err);
                }
            }
        }
    })
}

/// Broadcasts every user update to all subscribers
pub async fn subscribe_all(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        let mut rx = LOCATION_BROADCAST.subscribe();
        while let Ok((_user_id, json)) = rx.recv().await {
            let msg = json.to_string();
            if socket.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    })
}

/// Subscribes to updates for a specific user
pub async fn subscribe_one(Path(user_id): Path<String>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        let mut rx = LOCATION_BROADCAST.subscribe();
        while let Ok((id, json)) = rx.recv().await {
            if id == user_id {
                let msg = json.to_string();
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    })
}

/// Returns all currently tracked user locations
pub async fn get_all_users() -> impl IntoResponse {
    let map = LOCATION_MAP.read().unwrap();
    Json(map.clone())
}

/// Returns a single user’s last known location
pub async fn get_user(Path(user_id): Path<String>) -> impl IntoResponse {
    let map = LOCATION_MAP.read().unwrap();
    match map.get(&user_id) {
        Some(loc) => Json(loc.clone()).into_response(),
        None => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}

