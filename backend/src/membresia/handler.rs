use axum::Json;
use axum::extract::{Path, State};
use tracing::instrument;

use crate::app::errors::AppError;
use crate::app::state::AppState;
use crate::membresia;
use crate::membresia::dto::{CreateMembresiaRequest, MembresiaResponse};

#[instrument(name = "membresia.create", skip(state, request), err)]
pub async fn create_membresia_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateMembresiaRequest>,
) -> Result<Json<MembresiaResponse>, AppError> {
    let membresia = membresia::service::create(&state.db, request)
        .await
        .map_err(AppError::from)?;
    Ok(Json(MembresiaResponse::from(membresia)))
}

#[instrument(name = "membresia.get_by_id", skip(state), fields(id = %id), err)]
pub async fn get_membresia_by_id_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<MembresiaResponse>, AppError> {
    let membresia = membresia::service::get_by_id(&state.db, &id)
        .await
        .map_err(AppError::from)?;
    Ok(Json(MembresiaResponse::from(membresia)))
}

#[instrument(name = "membresia.get_by_dni_route", skip(state), fields(id = %id), err)]
pub async fn get_membresia_by_dni_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<MembresiaResponse>>, AppError> {
    let membresia = membresia::service::get_by_all_by_dni(&state.db, id)
        .await
        .map_err(AppError::from)?;
    Ok(Json(
        membresia.into_iter().map(MembresiaResponse::from).collect(),
    ))
}

#[instrument(name = "membresia.list", skip(state), err)]
pub async fn get_membresias_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<MembresiaResponse>>, AppError> {
    let membresias = membresia::service::get_all(&state.db)
        .await
        .map_err(AppError::from)?;
    Ok(Json(
        membresias
            .into_iter()
            .map(MembresiaResponse::from)
            .collect(),
    ))
}

#[instrument(name = "membresia.update", skip(state, request), fields(id = %id), err)]
pub async fn update_membresia_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateMembresiaRequest>,
) -> Result<Json<MembresiaResponse>, AppError> {
    let updated = membresia::service::update(&state.db, &id, request)
        .await
        .map_err(AppError::from)?;
    Ok(Json(MembresiaResponse::from(updated)))
}

#[instrument(name = "membresia.delete", skip(state), fields(id = %id), err)]
pub async fn delete_membresia_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    membresia::service::delete_by_id(&state.db, &id)
        .await
        .map_err(AppError::from)
}
