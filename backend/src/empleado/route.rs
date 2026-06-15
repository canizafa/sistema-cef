use super::handler::*;
use crate::app::state::AppState;
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn empleado_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_empleado_handler))
        .route(
            "/get-empleado-by-email/{id}",
            get(get_empleado_by_email_handler),
        )
        .route(
            "/get-empleado-by-dni/{id}",
            get(get_empleado_by_dni_handler),
        )
        .route("/update-empleado/{id}", put(update_empleado_handler))
        .route("/delete-empleado/{id}", delete(delete_empleado_handler))
        .route("/get-all", get(get_empleados_handler))
}
