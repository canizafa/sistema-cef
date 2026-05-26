use axum::{Json, extract::Path, extract::State};

use crate::{
    app_state::AppState, domain::Actividad, dtos::CreateActividadRequest, errors::AppError,
    repositories::ActividadRepository,
};

pub async fn create_actividad_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateActividadRequest>,
) -> Result<Json<Actividad>, AppError> {
    let actividad = Actividad::from(request);
    actividad.validate_actividad()?;
    ActividadRepository::create(&state.db, &actividad).await?;
    Ok(Json(actividad))
}

pub async fn get_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Actividad>, AppError> {
    let actividad = ActividadRepository::get_by_id(&state.db, &id).await?;
    Ok(Json(actividad))
}

pub async fn get_actividades_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Actividad>>, AppError> {
    let actividades = ActividadRepository::get_all(&state.db).await?;
    Ok(Json(actividades))
}

pub async fn delete_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Actividad>, AppError> {
    let actividad = ActividadRepository::delete(&state.db, &id).await?;
    Ok(Json(actividad))
}

pub async fn update_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateActividadRequest>,
) -> Result<Json<Actividad>, AppError> {
    let actividad = Actividad::from(request);
    actividad.validate_actividad()?;
    ActividadRepository::update(&state.db, &id, &actividad).await?;
    Ok(Json(actividad))
}
