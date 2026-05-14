use axum::{
    Json, Router,
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Alumno {
    //1 alumno
    nombre: String,
    apellido: String,
    dni: String,
    mail: String,
    //planilla deberia ser otra entidad
}
pub fn router() -> Router {
    Router::new()
        .route("/api/users/{id}", axum::routing::get(leer_alumno))
        .route("/alumnos", post(crear_alumno)) //GET
}
pub async fn leer_alumno(Path(user_id): Path<i32>) -> impl IntoResponse {
    format!("leyendo alumno {}", user_id) //aca iria la consulta sql
}
pub async fn crear_alumno(Json(json_persona): Json<Alumno>) -> impl IntoResponse {
    format!("creando alumno {}", json_persona.mail) //aca iria la consulta sqlito
}
