use crate::models::alumno::*;

use axum::{Json, Router, extract::Path, response::IntoResponse, routing::post};
use sqlx::SqlitePool;

pub fn alumnos_router() -> Router<SqlitePool> {
    Router::new()
        .route("/api/users/{id}", axum::routing::get(leer_alumno))
        .route("/alumnos", post(crear_alumno)) //GET
}
pub async fn leer_alumno(Path(user_id): Path<i32>) -> impl IntoResponse {
    format!("leyendo alumno {}", user_id) //aca iria la consulta sql
}
pub async fn crear_alumno(Json(json_persona): Json<CrearAlumno>) -> impl IntoResponse {
    format!("creando alumno {}", json_persona.mail) //aca iria la consulta sqlito
}
