use axum::{Json, extract::Path, extract::State};

use crate::{
    app_state::AppState,
    domain::Actividad,
    dtos::{ActividadResponse, CreateActividadRequest},
    errors::ApiError,
    repository::ActividadRepository,
};

#[axum::debug_handler]
pub async fn create_actividad_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateActividadRequest>,
) -> Result<Json<ActividadResponse>, ApiError> {
    let actividad = Actividad::from(request);
    actividad.validate_actividad()?;
    ActividadRepository::create_actividad(&state.db, &actividad).await?;
    Ok(Json(ActividadResponse::from(actividad)))
}

pub async fn get_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ActividadResponse>, ApiError> {
    let actividad = ActividadRepository::get_actividad_by_id(&state.db, &id).await?;
    Ok(Json(ActividadResponse::from(actividad)))
}

pub async fn get_actividades_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ActividadResponse>>, ApiError> {
    let actividades = ActividadRepository::get_all_actividades(&state.db).await?;
    Ok(Json(
        actividades
            .into_iter()
            .map(ActividadResponse::from)
            .collect(),
    ))
}

pub async fn delete_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ActividadResponse>, ApiError> {
    let actividad = ActividadRepository::delete_actividad(&state.db, &id).await?;
    Ok(Json(ActividadResponse::from(actividad)))
}

pub async fn update_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateActividadRequest>,
) -> Result<Json<ActividadResponse>, ApiError> {
    let actividad = Actividad::from(request);
    actividad.validate_actividad()?;
    ActividadRepository::update_actividad(&state.db, &id, &actividad).await?;
    Ok(Json(ActividadResponse::from(actividad)))
}
