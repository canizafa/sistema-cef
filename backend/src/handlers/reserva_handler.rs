use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::instrument;

use crate::{
    app_state::AppState,
    domain::Reserva,
    dtos::{CreateReservaRequest, ReservaResponse},
    errors::ApiError,
    repository::ReservaRepository,
};

#[instrument(name = "reserva.create", skip(state, body), err)]
pub async fn create_reserva_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateReservaRequest>,
) -> Result<Json<ReservaResponse>, ApiError> {
    let reserva = Reserva::from(body);
    reserva.validate_reserva()?;
    let reserva = ReservaRepository::create_reserva(&state.db, &reserva).await?;
    Ok(Json(ReservaResponse::from(reserva)))
}

#[instrument(name = "reserva.get", skip(state), fields(id = %id), err)]
pub async fn get_reserva_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservaResponse>, ApiError> {
    let reserva = ReservaRepository::get_reserva(&state.db, &id).await?;
    Ok(Json(ReservaResponse::from(reserva)))
}

#[instrument(name = "reserva.update", skip(state, body), fields(id = %id), err)]
pub async fn update_reserva_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<CreateReservaRequest>,
) -> Result<Json<ReservaResponse>, ApiError> {
    let reserva = Reserva::from(body);
    reserva.validate_reserva()?;
    let reserva = ReservaRepository::update_reserva(&state.db, &id, &reserva).await?;
    Ok(Json(ReservaResponse::from(reserva)))
}

#[instrument(name = "reserva.delete", skip(state), fields(id = %id), err)]
pub async fn delete_reserva_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    ReservaRepository::delete_reserva(&state.db, &id).await?;
    Ok(StatusCode::OK)
}

#[instrument(name = "reserva.list", skip(state), err)]
pub async fn get_reservas_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ReservaResponse>>, ApiError> {
    let reservas = ReservaRepository::get_all(&state.db).await?;
    Ok(Json(
        reservas.into_iter().map(ReservaResponse::from).collect(),
    ))
}
