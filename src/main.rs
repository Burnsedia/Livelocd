// src/main.rs
use axum::{extract::Path, extract::ws::{Message, WebSocketUpgrade}, response::IntoResponse, routing::get, Json, Router};
use futures::StreamExt;
use once_cell::sync::Lazy;
use serde_json::json;
use std::{collections::HashMap, net::SocketAddr, sync::RwLock};
use tokio::sync::broadcast;
use tokio::net::TcpListener;

static LOCATION_MAP: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| RwLock::new(HashMap::new()));
static LOCATION_BROADCAST: Lazy<broadcast::Sender<(String, String)>> = Lazy::new(|| broadcast::channel(1024).0);

async fn send_location(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        while let Some(Ok(Message::Text(text))) = socket.next().await {
            if let Some((user_id, coords)) = text.split_once(":") {
                LOCATION_MAP.write().unwrap().insert(user_id.to_string(), coords.to_string());
                let _ = LOCATION_BROADCAST.send((user_id.to_string(), coords.to_string()));
            }
        }
    })
}

async fn subscribe_all(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        let mut rx = LOCATION_BROADCAST.subscribe();
        while let Ok((user_id, coords)) = rx.recv().await {
            let msg = format!("{}:{}", user_id, coords);
            if socket.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    })
}

async fn subscribe_one(Path(user_id): Path<String>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        let mut rx = LOCATION_BROADCAST.subscribe();
        while let Ok((id, coords)) = rx.recv().await {
            if id == user_id {
                let msg = format!("{}:{}", id, coords);
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    })
}

async fn get_all_users() -> impl IntoResponse {
    let map = LOCATION_MAP.read().unwrap();
    Json(map.clone())
}

async fn get_user(Path(user_id): Path<String>) -> impl IntoResponse {
    let map = LOCATION_MAP.read().unwrap();
    match map.get(&user_id) {
        Some(loc) => Json(json!({ "user_id": user_id, "location": loc })).into_response(),
        None => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws/send-location", get(send_location))
        .route("/ws/subscribe", get(subscribe_all))
        .route("/ws/subscribe/:user_id", get(subscribe_one))
        .route("/api/users", get(get_all_users))
        .route("/api/users/:user_id", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

