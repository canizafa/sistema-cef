use super::*;
use crate::app::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn asistencia_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_asistencia_handler))
        .route("/get-asistencia/{id}", get(get_asistencia_by_id_handler))
        .route("/update-asistencia/{id}", put(update_asistencia_handler))
        .route("/delete-asistencia/{id}", delete(delete_asistencia_handler))
        .route("/get-all", get(get_asistencias_handler))
}
