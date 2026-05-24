use axum::{Json, extract::State};
use sqlx::SqlitePool;

use crate::dtos::CreateAsistenciaRequest;

pub async fn create_asistencia_handler(
    State(pool): State<SqlitePool>,
    Json(CreateAsistenciaRequest): Json<CreateAsistenciaRequest>,
) {
}

pub async fn get_asistencia_handler() {
    todo!()
}

pub async fn update_asistencia_handler() {
    todo!()
}

pub async fn delete_asistencia_handler() {
    todo!()
}

pub async fn get_asistencias_handler() {
    todo!()
}
