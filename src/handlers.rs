use crate::state::{LOCATION_BROADCAST, LOCATION_MAP, TOKEN_VALIDATOR};
use axum::{
    extract::{
        ws::{Message, WebSocketUpgrade},
        Path,
        Query,
    },
    response::IntoResponse,
    Json,
};
use futures::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct AuthQuery {
    token: String,
}

fn get_validator() -> &'static Arc<dyn crate::auth::TokenValidator> {
    TOKEN_VALIDATOR
        .get()
        .expect("Token validator has not been set. Please call `set_token_validator` first.")
}

/// Accepts WebSocket location updates as JSON
/// Expected format: `{ "user_id": "user123", "location": "33.7,-84.3" }`
pub async fn send_location(ws: WebSocketUpgrade, Query(auth): Query<AuthQuery>) -> impl IntoResponse {
    if !get_validator().validate(&auth.token).await {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    }

    ws.on_upgrade(|mut socket| async move {
        while let Some(Ok(Message::Text(text))) = socket.next().await {
            match serde_json::from_str::<Value>(&text) {
                Ok(json) => {
                    if let Some(user_id) = json.get("user_id").and_then(|v| v.as_str()) {
                        LOCATION_MAP
                            .write()
                            .unwrap()
                            .insert(user_id.to_string(), json.clone());
                        let _ = LOCATION_BROADCAST.send((user_id.to_string(), json));
                    }
                }
                Err(err) => {
                    eprintln!("❌ Invalid JSON: {}", err);
                }
            }
        }
    })
    .into_response()
}

/// Broadcasts every user update to all subscribers
pub async fn subscribe_all(
    ws: WebSocketUpgrade,
    Query(auth): Query<AuthQuery>,
) -> impl IntoResponse {
    if !get_validator().validate(&auth.token).await {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    }

    ws.on_upgrade(|mut socket| async move {
        let mut rx = LOCATION_BROADCAST.subscribe();
        while let Ok((_user_id, json)) = rx.recv().await {
            let msg = json.to_string();
            if socket.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    })
    .into_response()
}

/// Subscribes to updates for a specific user
pub async fn subscribe_one(
    Path(user_id): Path<String>,
    ws: WebSocketUpgrade,
    Query(auth): Query<AuthQuery>,
) -> impl IntoResponse {
    if !get_validator().validate(&auth.token).await {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    }

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
    .into_response()
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
