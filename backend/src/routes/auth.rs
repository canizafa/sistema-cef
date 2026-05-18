use axum::{Json, Router, extract::Path, response::IntoResponse};

use crate::models::alumno::CrearAlumno;

pub fn auth_router() -> Router {
    Router::new()
        .route("/api/auth/register", axum::routing::post(registrar_usuario))
        .route("/api/auth/login", axum::routing::post(login_usuario))
}

pub async fn registrar_usuario(Json(usuario): Json<CrearAlumno>) -> impl IntoResponse {
    format!("Se creo el usuario: {:?}", usuario)
}

pub async fn login_usuario(Path(user_id): Path<i32>) -> impl IntoResponse {
    format!("Creaste al usuario con ID: {}", user_id)
}
