use axum::Router;
use axum::routing::{post, put};

use crate::app_state::AppState;
use crate::handlers::change_password_handler;
use crate::handlers::login_handler;
use crate::handlers::register_cliente_handler;
use crate::handlers::register_empleado_handler;
use crate::handlers::reset_password_handler;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/change-password/{dni}", put(change_password_handler))
        .route("/reset-password", post(reset_password_handler))
        .route("/register-empleado", post(register_empleado_handler))
        .route("/register-cliente", post(register_cliente_handler))
}
