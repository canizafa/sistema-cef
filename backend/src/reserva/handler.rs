use crate::{
    app::{errors::AppError, state::AppState},
    reserva::{
        self,
        dto::{CreateReservaRequest, ReservaResponse},
    },
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::instrument;

#[instrument(name = "reserva.create", skip(state, request), err)]
pub async fn create_reserva_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateReservaRequest>,
) -> Result<Json<ReservaResponse>, AppError> {
    let reserva = reserva::service::create(&state.db, request).await?;
    Ok(Json(ReservaResponse::from(reserva)))
}

#[instrument(name = "reserva.get", skip(state), fields(id = %id), err)]
pub async fn get_reserva_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservaResponse>, AppError> {
    let reserva = reserva::service::get_by_id(&state.db, &id).await?;
    Ok(Json(ReservaResponse::from(reserva)))
}

#[instrument(name = "reserva.update", skip(state, body), fields(id = %id), err)]
pub async fn update_reserva_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<CreateReservaRequest>,
) -> Result<Json<ReservaResponse>, AppError> {
    let reserva = reserva::service::update(&state.db, &id, body).await?;
    Ok(Json(ReservaResponse::from(reserva)))
}

#[instrument(name = "reserva.delete", skip(state), fields(id = %id), err)]
pub async fn delete_reserva_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    reserva::service::delete_reserva(&state.db, &id).await?;
    Ok(StatusCode::OK)
}

#[instrument(name = "reserva.list", skip(state), err)]
pub async fn get_reservas_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ReservaResponse>>, AppError> {
    let reservas = reserva::service::get_all(&state.db).await?;
    Ok(Json(
        reservas.into_iter().map(ReservaResponse::from).collect(),
    ))
}
