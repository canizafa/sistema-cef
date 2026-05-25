use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    app_state::AppState, domain::Reserva, dtos::CreateReservaRequest, errors::ApiError,
    repository::ReservaRepository,
};

pub async fn create_reserva_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateReservaRequest>,
) -> Result<Json<Reserva>, ApiError> {
    let reserva = Reserva::from(body);
    reserva.validate_reserva()?;
    let reserva = ReservaRepository::create_reserva(&state.db, &reserva).await?;
    Ok(Json(reserva))
}

pub async fn get_reserva_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Reserva>, ApiError> {
    let reserva = ReservaRepository::get_reserva(&state.db, &id).await?;
    Ok(Json(reserva))
}

pub async fn update_reserva_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<CreateReservaRequest>,
) -> Result<Json<Reserva>, ApiError> {
    let reserva = Reserva::from(body);
    reserva.validate_reserva()?;
    let reserva = ReservaRepository::update_reserva(&state.db, &id, &reserva).await?;
    Ok(Json(reserva))
}

pub async fn delete_reserva_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    ReservaRepository::delete_reserva(&state.db, &id).await?;
    Ok(StatusCode::OK)
}

pub async fn get_reservas_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Reserva>>, ApiError> {
    let reservas = ReservaRepository::get_all(&state.db).await?;
    Ok(Json(reservas))
}
