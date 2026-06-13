use super::*;
use crate::app::*;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::instrument;

#[instrument(name = "membresia.create", skip(state, request), err)]
pub async fn create_membresia_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateMembresiaRequest>,
) -> Result<Json<MembresiaResponse>, ApiError> {
    let membresia = Membresia::from(request);
    membresia.validate_membresia()?;
    let created = MembresiaRepository::create_membresia(&state.db, &membresia).await?;
    Ok(Json(MembresiaResponse::from(created)))
}

#[instrument(name = "membresia.get_by_id", skip(state), fields(id = %id), err)]
pub async fn get_membresia_by_id_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<MembresiaResponse>, ApiError> {
    let membresia = MembresiaRepository::get_by_id(&state.db, &id).await?;
    Ok(Json(MembresiaResponse::from(membresia)))
}

#[instrument(name = "membresia.get_by_dni_route", skip(state), fields(id = %id), err)]
pub async fn get_membresia_by_dni_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<MembresiaResponse>, ApiError> {
    let membresia = MembresiaRepository::get_by_id(&state.db, &id).await?;
    Ok(Json(MembresiaResponse::from(membresia)))
}

#[instrument(name = "membresia.list", skip(state), err)]
pub async fn get_membresias_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<MembresiaResponse>>, ApiError> {
    let membresias = MembresiaRepository::get_all(&state.db).await?;
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
) -> Result<Json<MembresiaResponse>, ApiError> {
    let membresia = Membresia::from(request);
    membresia.validate_membresia()?;
    let updated = MembresiaRepository::update_membresia(&state.db, &id, &membresia).await?;
    Ok(Json(MembresiaResponse::from(updated)))
}

#[instrument(name = "membresia.delete", skip(state), fields(id = %id), err)]
pub async fn delete_membresia_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    MembresiaRepository::delete_membresia(&state.db, &id).await?;
    Ok(StatusCode::OK.into_response())
}
