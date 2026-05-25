use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use crate::{
    app_state::AppState,
    domain::Clase,
    dtos::{ClaseResponse, CreateClaseRequest},
    errors::ApiError,
    repository::ClaseRepository,
};

pub async fn create_clase_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateClaseRequest>,
) -> Result<Json<ClaseResponse>, ApiError> {
    let clase = Clase::from(request);
    clase.validate_clase()?;
    ClaseRepository::create_clase(&state.db, &clase).await?;
    Ok(Json(ClaseResponse::from(clase)))
}

pub async fn get_clase_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ClaseResponse>, ApiError> {
    let clase = ClaseRepository::get_by_id(&state.db, &id).await?;
    if let Some(c) = clase {
        Ok(Json(ClaseResponse::from(c)))
    } else {
        Err(ApiError::NotFound)
    }
}

pub async fn update_clase_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateClaseRequest>,
) -> Result<Json<ClaseResponse>, ApiError> {
    let clase = Clase::from(request);
    clase.validate_clase()?;
    ClaseRepository::update_clase(&state.db, &id, &clase).await?;
    Ok(Json(ClaseResponse::from(clase)))
}

pub async fn delete_clase_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    ClaseRepository::delete_clase(&state.db, &id).await?;
    Ok(StatusCode::OK)
}

pub async fn get_clases_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ClaseResponse>>, ApiError> {
    let clases = ClaseRepository::get_all(&state.db).await?;
    Ok(Json(clases.into_iter().map(ClaseResponse::from).collect()))
}
