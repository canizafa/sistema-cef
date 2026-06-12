use super::*;
use crate::app::{ApiError, AppState};
use axum::extract::Path;
use axum::{Json, extract::State};
use tracing::instrument;

#[instrument(name = "asistencia.create", skip(pool, request), err)]
pub async fn create_asistencia_handler(
    State(pool): State<AppState>,
    Json(request): Json<CreateAsistenciaRequest>,
) -> Result<Json<AsistenciaResponse>, ApiError> {
    let asistencia = Asistencia::from(request);
    asistencia.validate_asistencia()?;
    let created = AsistenciaRepository::create_asistencia(&pool.db, &asistencia).await?;
    Ok(Json(AsistenciaResponse::from(created)))
}

#[instrument(name = "asistencia.get", skip(pool), fields(id = %id), err)]
pub async fn get_asistencia_by_id_handler(
    State(pool): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AsistenciaResponse>, ApiError> {
    let asistencia = AsistenciaRepository::get_asistencia_by_id(&pool.db, &id).await?;
    Ok(Json(AsistenciaResponse::from(asistencia)))
}

#[instrument(name = "asistencia.update", skip(pool, request), fields(id = %id), err)]
pub async fn update_asistencia_handler(
    State(pool): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateAsistenciaRequest>,
) -> Result<Json<AsistenciaResponse>, ApiError> {
    let asistencia = Asistencia::from(request);
    asistencia.validate_asistencia()?;
    let updated = AsistenciaRepository::update_asistencia(&pool.db, &id, &asistencia).await?;
    Ok(Json(AsistenciaResponse::from(updated)))
}

#[instrument(name = "asistencia.delete", skip(pool), fields(id = %id), err)]
pub async fn delete_asistencia_handler(
    State(pool): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AsistenciaResponse>, ApiError> {
    let deleted = AsistenciaRepository::delete_asistencia(&pool.db, &id).await?;
    Ok(Json(AsistenciaResponse::from(deleted)))
}

#[instrument(name = "asistencia.list", skip(pool), err)]
pub async fn get_asistencias_handler(
    State(pool): State<AppState>,
) -> Result<Json<Vec<AsistenciaResponse>>, ApiError> {
    let asistencias = AsistenciaRepository::get_all_asistencias(&pool.db).await?;
    Ok(Json(
        asistencias
            .into_iter()
            .map(|a| AsistenciaResponse::from(a))
            .collect(),
    ))
}
