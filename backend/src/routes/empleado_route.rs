use axum::Router;
use axum::routing::{delete, get, post, put};

use crate::app_state::AppState;
use crate::handlers::{
    create_empleado_handler, delete_empleado_handler, get_empleado_handler, get_empleados_handler,
    update_empleado_handler,
};

pub fn empleado_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_empleado_handler))
        .route("/get-empleado/{id}", get(get_empleado_handler))
        .route("/update-empleado/{id}", put(update_empleado_handler))
        .route("/delete-empleado/{id}", delete(delete_empleado_handler))
        .route("/get-all", get(get_empleados_handler))
}
