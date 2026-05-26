use axum::routing::{delete, put};
use axum::{Router, routing::get, routing::post};

use crate::app_state::AppState;
use crate::handlers::create_actividad_handler;
use crate::handlers::delete_actividad_handler;
use crate::handlers::get_actividad_handler;
use crate::handlers::get_actividades_handler;
use crate::handlers::update_actividad_handler;

pub fn actividad_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_actividad_handler))
        .route("/get-actividad/{id}", get(get_actividad_handler))
        .route("/get-actividades", get(get_actividades_handler))
        .route("/delete/{id}", delete(delete_actividad_handler))
        .route("/update/{id}", put(update_actividad_handler))
}
