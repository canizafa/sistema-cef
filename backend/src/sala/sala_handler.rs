use super::*;
use crate::app::{ApiError, AppState};
use axum::{Json, extract::Path, extract::State};
use tracing::instrument;

#[instrument(name = "sala.create", skip(state, request), err)]
pub async fn create_sala_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateSalaRequest>,
) -> Result<Json<SalaResponse>, ApiError> {
    let sala = Sala::from(request);
    sala.validate_sala()?;
    SalaRepository::create_sala(&state.db, &sala).await?;
    Ok(Json(SalaResponse::from(sala)))
}

#[instrument(name = "sala.get", skip(state), fields(id = %id), err)]
pub async fn get_sala_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<SalaResponse>, ApiError> {
    let sala = SalaRepository::get_sala_by_id(&state.db, &id).await?;
    Ok(Json(SalaResponse::from(sala)))
}

#[instrument(name = "sala.list", skip(state), err)]
pub async fn get_salas_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<SalaResponse>>, ApiError> {
    let salas = SalaRepository::get_all_salas(&state.db).await?;
    Ok(Json(salas.into_iter().map(SalaResponse::from).collect()))
}

#[instrument(name = "sala.update", skip(state, request), fields(id = %id), err)]
pub async fn update_sala_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateSalaRequest>,
) -> Result<Json<SalaResponse>, ApiError> {
    let sala = Sala::from(request);
    sala.validate_sala()?;
    SalaRepository::update_sala(&state.db, &id, &sala).await?;
    Ok(Json(SalaResponse::from(sala)))
}

#[instrument(name = "sala.delete", skip(state), fields(id = %id), err)]
pub async fn delete_sala_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(), ApiError> {
    SalaRepository::delete_sala(&state.db, &id).await?;
    Ok(())
}
