use crate::AppState;
use axum::Router;
use axum::routing::{get, post};

use crate::handlers::{
    delete_empleado_handler, get_empleado_handler, get_empleados_handler, update_empleado_handler,
};

pub fn empleado_router() -> Router<AppState> {
    Router::new()
        .route("/get-empleado", get(get_empleado_handler))
        .route("/update-empleado", post(update_empleado_handler))
        .route("/delete-empleado", post(delete_empleado_handler))
        .route("/get-all", get(get_empleados_handler))
}
