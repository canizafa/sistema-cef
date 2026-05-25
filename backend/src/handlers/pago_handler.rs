use axum::extract::{Json, Path, State};

use crate::{
    app_state::AppState,
    domain::pago::Pago,
    dtos::{CreatePagoRequest, PagoResponse},
    errors::ApiError,
    repository::PagoRepository,
};

pub async fn create_pago_handler(
    State(state): State<AppState>,
    Json(body): Json<CreatePagoRequest>,
) -> Result<Json<PagoResponse>, ApiError> {
    let pago = Pago::from(body);
    let pago = PagoRepository::create_pago(&state.db, &pago).await?;
    Ok(Json(PagoResponse::from(pago)))
}

pub async fn get_pago_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<PagoResponse>, ApiError> {
    let pago = PagoRepository::get_pago(&state.db, &id).await?;
    if let Some(pago) = pago {
        Ok(Json(PagoResponse::from(pago)))
    } else {
        Err(ApiError::NotFound)
    }
}

pub async fn update_pago_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<CreatePagoRequest>,
) -> Result<Json<PagoResponse>, ApiError> {
    let pago = Pago::from(body);
    let pago = PagoRepository::update_pago(&state.db, &id, &pago).await?;
    if let Some(pago) = pago {
        Ok(Json(PagoResponse::from(pago)))
    } else {
        Err(ApiError::NotFound)
    }
}

pub async fn delete_pago_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<PagoResponse>, ApiError> {
    let pago = PagoRepository::delete_pago(&state.db, &id).await?;
    if let Some(pago) = pago {
        Ok(Json(PagoResponse::from(pago)))
    } else {
        Err(ApiError::NotFound)
    }
}

pub async fn get_pagos_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<PagoResponse>>, ApiError> {
    let pagos = PagoRepository::get_all(&state.db).await?;
    if let Some(pagos) = pagos {
        Ok(Json(pagos.into_iter().map(PagoResponse::from).collect()))
    } else {
        Err(ApiError::NotFound)
    }
}
