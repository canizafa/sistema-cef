use axum::{Router, routing::get};
use sqlx::SqlitePool;

use crate::routes::{auth_route::auth_router, health_checker::health_checker};

pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/api", get(health_checker))
        .nest("/api/alumnos", alumnos_router()) // se anidan las rutas de alumnos que se trabajan en su propio archivo
        .nest("/api/auth", auth_router())
}
