use axum::extract::Path;
use axum::{Json, extract::State};

use crate::domain::asistencia::Asistencia;
use crate::{
    app_state::AppState,
    dtos::{AsistenciaResponse, CreateAsistenciaRequest},
    errors::ApiError,
    repository::AsistenciaRepository,
};

pub async fn create_asistencia_handler(
    State(pool): State<AppState>,
    Json(request): Json<CreateAsistenciaRequest>,
) -> Result<Json<AsistenciaResponse>, ApiError> {
    let asistencia = Asistencia::from(request);
    asistencia.validate_asistencia()?;
    let created = AsistenciaRepository::create_asistencia(&pool.db, &asistencia).await?;
    Ok(Json(AsistenciaResponse::from(created)))
}

pub async fn get_asistencia_by_id_handler(
    State(pool): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AsistenciaResponse>, ApiError> {
    let asistencia = AsistenciaRepository::get_asistencia_by_id(&pool.db, &id).await?;
    Ok(Json(AsistenciaResponse::from(asistencia)))
}

pub async fn update_asistencia_handler(
    State(pool): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateAsistenciaRequest>,
) -> Result<Json<AsistenciaResponse>, ApiError> {
    let asistencia = Asistencia::from(request);
    asistencia.validate_asistencia()?;
    let updated = AsistenciaRepository::update_asistencia(
        &pool.db,
        &asistencia.get_id_asistencia(),
        &asistencia,
    )
    .await?;
    Ok(Json(AsistenciaResponse::from(updated)))
}

pub async fn delete_asistencia_handler(
    State(pool): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AsistenciaResponse>, ApiError> {
    let deleted = AsistenciaRepository::delete_asistencia(&pool.db, &id).await?;
    Ok(Json(AsistenciaResponse::from(deleted)))
}

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
