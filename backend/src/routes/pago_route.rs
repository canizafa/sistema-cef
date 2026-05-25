use crate::app_state::AppState;
use crate::handlers::crear_pago_handler;

use axum::Router;
use axum::routing::post;

pub fn pago_router() -> Router<AppState> {
    Router::new().route("/create", post(crear_pago_handler))
}
