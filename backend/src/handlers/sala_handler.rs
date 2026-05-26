use axum::{Json, extract::Path, extract::State};

use crate::{
    app_state::AppState,
    domain::sala::Sala,
    dtos::sala_dto::{CreateSalaRequest, SalaResponse},
    errors::ApiError,
    repository::SalaRepository,
};

pub async fn create_sala_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateSalaRequest>,
) -> Result<Json<SalaResponse>, ApiError> {
    let sala = Sala::from(request);
    sala.validate_sala()?;
    SalaRepository::create_sala(&state.db, &sala).await?;
    Ok(Json(SalaResponse::from(sala)))
}

pub async fn get_sala_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<SalaResponse>, ApiError> {
    let sala = SalaRepository::get_sala_by_id(&state.db, &id).await?;
    Ok(Json(SalaResponse::from(sala)))
}

pub async fn get_salas_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<SalaResponse>>, ApiError> {
    let salas = SalaRepository::get_all_salas(&state.db).await?;
    Ok(Json(salas.into_iter().map(SalaResponse::from).collect()))
}

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

pub async fn delete_sala_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(), ApiError> {
    SalaRepository::delete_sala(&state.db, &id).await?;
    Ok(())
}
