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
