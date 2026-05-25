use axum::{Json, extract::State};

use crate::{
    app_state::AppState,
    dtos::{AsistenciaResponse, CreateAsistenciaRequest},
    errors::ApiError,
};

pub async fn create_asistencia_handler(
    State(pool): State<AppState>,
    Json(request): Json<CreateAsistenciaRequest>,
) -> Result<Json<AsistenciaResponse>, ApiError> {
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
