use axum::{Json, Router, extract::Path, response::IntoResponse};
use sqlx::SqlitePool;

pub fn auth_router() -> Router<SqlitePool> {
    Router::new()
        .route("/register", axum::routing::post(registrar_usuario))
        .route("/login/{id}", axum::routing::post(login_usuario))
}

pub async fn registrar_usuario(Json(usuario): Json<Cliente>) -> impl IntoResponse {
    format!("Se creo el usuario: {:?}", usuario)
}

pub async fn login_usuario(Path(user_id): Path<i32>) -> impl IntoResponse {
    format!("Creaste al usuario con ID: {}", user_id)
}
