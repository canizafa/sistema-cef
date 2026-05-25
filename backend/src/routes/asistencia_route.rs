use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    app_state::AppState,
    handlers::{
        create_asistencia_handler, delete_asistencia_handler, get_asistencia_by_id_handler,
        get_asistencias_handler, update_asistencia_handler,
    },
};

pub fn asistencia_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_asistencia_handler))
        .route("/get-asistencia/{id}", get(get_asistencia_by_id_handler))
        .route("/update-asistencia/{id}", post(update_asistencia_handler))
        .route("/delete-asistencia/{id}", post(delete_asistencia_handler))
        .route("/get-all", get(get_asistencias_handler))
}
