use super::handler::notify_handler;
use crate::{app::state::AppState, notificaciones::handler::update_notify_date_client};
use axum::{
    Router,
    routing::{post, put},
};

pub fn notificaciones_route() -> Router<AppState> {
    Router::new()
        .route("/notify", post(notify_handler))
        .route("/update-date", put(update_notify_date_client))
}
