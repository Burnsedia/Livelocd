use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use futures_util::sink::SinkExt;
use livelocd::{auth::AllowAllValidator, livelocd_routes, set_token_validator};
use serde_json::json;
use std::sync::Arc;
use tower::util::ServiceExt;

async fn send_location_update(user_id: &str, location: &str) {
    let app = livelocd_routes();
    let server = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = server.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(server, app.into_make_service())
            .await
            .unwrap();
    });

    let (mut socket, _response) = tokio_tungstenite::connect_async(
        format!("ws://{}/ws/send-location?token=test", addr),
    )
    .await
    .unwrap();

    let msg = json!({ "user_id": user_id, "location": location }).to_string();
    socket
        .send(tokio_tungstenite::tungstenite::Message::Text(msg))
        .await
        .unwrap();
}

#[tokio::test]
async fn get_all_users_returns_empty_list_initially() {
    set_token_validator(Arc::new(AllowAllValidator));
    let app = livelocd_routes();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/users")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(&body[..], b"{}");
}

#[tokio::test]
async fn get_user_returns_not_found_for_unknown_user() {
    set_token_validator(Arc::new(AllowAllValidator));
    let app = livelocd_routes();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/users/unknown_user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_user_returns_location_for_known_user() {
    set_token_validator(Arc::new(AllowAllValidator));
    send_location_update("user1", "12.34,56.78").await;

    let app = livelocd_routes();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/users/user1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({ "user_id": "user1", "location": "12.34,56.78" }));
}

#[tokio::test]
async fn get_all_users_returns_list_of_users() {
    set_token_validator(Arc::new(AllowAllValidator));
    send_location_update("user2", "-12.34,-56.78").await;

    let app = livelocd_routes();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/users")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(body.as_object().unwrap().contains_key("user2"));
}
