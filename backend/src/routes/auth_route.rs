use axum::Router;
use axum::routing::post;

use crate::AppState;
use crate::handlers::login_handler;
use crate::handlers::logout_handler;
use crate::handlers::register_empleado_handler;
use crate::handlers::register_handler;
use crate::handlers::reset_password_handler;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/change-password", post(reset_password_handler))
        .route("/register", post(register_handler))
        .route("/register-empleado", post(register_empleado_handler))
}
