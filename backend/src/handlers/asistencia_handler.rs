use axum::{Json, extract::State};

use crate::{app_state::AppState, dtos::CreateAsistenciaRequest};

pub async fn create_asistencia_handler(
    State(pool): State<AppState>,
    Json(CreateAsistenciaRequest): Json<CreateAsistenciaRequest>,
) {
    todo!()
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
