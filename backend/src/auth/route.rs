use super::handler::*;
use crate::app::state::AppState;
use axum::Router;
use axum::routing::{post, put};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/change-password/{dni}", put(change_password_handler))
        .route("/reset-password", post(reset_password_handler))
}
