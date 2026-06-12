use super::*;
use crate::app::AppState;
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn empleado_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_empleado_handler))
        .route("/get-empleado/{id}", get(get_empleado_handler))
        .route("/update-empleado/{id}", put(update_empleado_handler))
        .route("/delete-empleado/{id}", delete(delete_empleado_handler))
        .route("/get-all", get(get_empleados_handler))
}
