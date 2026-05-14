use axum::{Router, routing::get};

use crate::routes::{alumnos::alumnos_router, health_checker::health_checker};

pub fn router() -> Router {
    Router::new()
        .route("/", get(health_checker))
        .nest("/alumnos", alumnos_router()) // se anidan las rutas de alumnos que se trabajan en su propio archivo
}
