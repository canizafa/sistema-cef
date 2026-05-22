use axum::Router;
use sqlx::SqlitePool;

pub fn router() -> Router<SqlitePool> {
    Router::new();
    todo!()
    // .route("/api", get(health_checker))
    // .nest("/api/clientes", clientes_router())
    // .nest("/api/auth", auth_router())
}
