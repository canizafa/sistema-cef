use super::handler::{
    create_actividad_handler, delete_actividad_handler, get_actividad_handler,
    get_actividades_handler, update_actividad_handler,
};
use crate::app::state::AppState;
use axum::routing::{delete, put};
use axum::{Router, routing::get, routing::post};

pub fn actividad_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_actividad_handler))
        .route("/get-actividad/{id}", get(get_actividad_handler))
        .route("/get-actividades", get(get_actividades_handler))
        .route("/delete/{id}", delete(delete_actividad_handler))
        .route("/update/{id}", put(update_actividad_handler))
}
