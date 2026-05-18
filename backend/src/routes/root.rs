use axum::{Router, routing::get};

use crate::routes::{alumnos::alumnos_router, auth::auth_router, health_checker::health_checker};

pub fn router() -> Router {
    Router::new()
        .route("/api", get(health_checker))
        .nest("/api/", alumnos_router()) // se anidan las rutas de alumnos que se trabajan en su propio archivo
        .nest("/api/auth", auth_router())
}
