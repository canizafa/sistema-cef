use crate::app::{errors::AppError, state::AppState};
use crate::asistencia;
use crate::asistencia::dto::{AsistenciaResponse, CreateAsistenciaRequest};
use axum::extract::Path;
use axum::{Json, extract::State};
use tracing::instrument;

#[instrument(name = "asistencia.create", skip(state, request), err)]
pub async fn create_asistencia_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateAsistenciaRequest>,
) -> Result<Json<AsistenciaResponse>, AppError> {
    let asistencia = asistencia::service::create(&state.db, request).await?;
    Ok(Json(AsistenciaResponse::from(asistencia)))
}

#[instrument(name = "asistencia.get", skip(state), fields(id = %id), err)]
pub async fn get_asistencia_by_id_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AsistenciaResponse>, AppError> {
    let asistencia = asistencia::service::get_by_id(&state.db, &id).await?;
    Ok(Json(AsistenciaResponse::from(asistencia)))
}

#[instrument(name = "asistencia.update", skip(state, request), fields(id = %id), err)]
pub async fn update_asistencia_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateAsistenciaRequest>,
) -> Result<Json<AsistenciaResponse>, AppError> {
    let asistencia = asistencia::service::update(&state.db, &id, request).await?;
    Ok(Json(AsistenciaResponse::from(asistencia)))
}

#[instrument(name = "asistencia.delete", skip(state), fields(id = %id), err)]
pub async fn delete_asistencia_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    asistencia::service::delete(&state.db, &id).await
}
