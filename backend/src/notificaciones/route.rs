use super::handler::notify_handler;
use crate::app::state::AppState;
use axum::{Router, routing::post};

pub fn notificaciones_route() -> Router<AppState> {
    Router::new().route("/", post(notify_handler))
}
