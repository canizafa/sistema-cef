use axum::{Json, extract::Path, extract::State};
use tracing::instrument;

use crate::app::{AppError, AppState};

use super::{Actividad, ActividadRepository, ActividadResponse, CreateActividadRequest};

#[instrument(name = "actividad.create", skip(state, request), err)]
pub async fn create_actividad_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateActividadRequest>,
) -> Result<Json<ActividadResponse>, AppError> {
    let actividad = Actividad::from(request);
    actividad.validate_actividad()?;
    ActividadRepository::create_actividad(&state.db, &actividad).await?;
    Ok(Json(ActividadResponse::from(actividad)))
}

#[instrument(name = "actividad.get", skip(state), fields(id = %id), err)]
pub async fn get_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ActividadResponse>, AppError> {
    let actividad = ActividadRepository::get_actividad_by_id(&state.db, &id).await?;
    Ok(Json(ActividadResponse::from(actividad)))
}

#[instrument(name = "actividad.list", skip(state), err)]
pub async fn get_actividades_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ActividadResponse>>, AppError> {
    let actividades = ActividadRepository::get_all_actividades(&state.db).await?;
    Ok(Json(
        actividades
            .into_iter()
            .map(ActividadResponse::from)
            .collect(),
    ))
}

#[instrument(name = "actividad.delete", skip(state), fields(id = %id), err)]
pub async fn delete_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ActividadResponse>, AppError> {
    let actividad = ActividadRepository::delete_actividad(&state.db, &id).await?;
    Ok(Json(ActividadResponse::from(actividad)))
}

#[instrument(name = "actividad.update", skip(state, request), fields(id = %id), err)]
pub async fn update_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateActividadRequest>,
) -> Result<Json<ActividadResponse>, AppError> {
    let actividad = Actividad::from(request);
    actividad.validate_actividad()?;
    ActividadRepository::update_actividad(&state.db, &id, &actividad).await?;
    Ok(Json(ActividadResponse::from(actividad)))
}
