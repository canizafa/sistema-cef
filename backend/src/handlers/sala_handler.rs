use axum::{Json, extract::Path, extract::State};

use crate::{
    app_state::AppState, domain::sala::Sala, dtos::sala_dto::CreateSalaRequest, errors::ApiError,
    repositories::sala_repository::SalaRepository,
};

pub async fn create_sala_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateSalaRequest>,
) -> Result<Json<Sala>, ApiError> {
    let sala = Sala::from(request);
    sala.validate_sala()?;
    SalaRepository::create(&state.db, sala).await?;
    Ok(Json(sala))
}

pub async fn get_sala_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Sala>, ApiError> {
    let sala = SalaRepository::get_by_id(&state.db, id).await?;
    Ok(Json(sala))
}

pub async fn get_salas_handler(State(state): State<AppState>) -> Result<Json<Vec<Sala>>, ApiError> {
    let salas = SalaRepository::get_all(&state.db).await?;
    Ok(Json(salas))
}

pub async fn update_sala_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateSalaRequest>,
) -> Result<Json<Sala>, ApiError> {
    let sala = Sala::from(request);
    sala.validate_sala()?;
    SalaRepository::update(&state.db, id, sala).await?;
    Ok(Json(sala))
}

pub async fn delete_sala_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(), ApiError> {
    SalaRepository::delete(&state.db, id).await?;
    Ok(())
}
