use axum::Router;
use axum::routing::get;
use axum::routing::post;

use crate::app_state::AppState;
use crate::handlers::create_actividad_handler;
use crate::handlers::get_actividad_handler;

pub fn actividad_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_actividad_handler))
        .route("/get-actividad/{id}", get(get_actividad_handler))
}
