use super::dto::{ClienteResponse, CreateClienteRequest};
use crate::app::{errors::AppError, state::AppState};
use crate::cliente;
use crate::cliente::dto::{ClienteRequest, EliminarClienteRequest, UpdatePasswordRequest};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use tracing::instrument;

#[instrument(name = "cliente.create", skip(state, request), fields(dni = request.dni), err)]
pub async fn create_cliente_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateClienteRequest>,
) -> Result<Json<ClienteResponse>, AppError> {
    let cliente = cliente::service::create(&state.db, request)
        .await
        .map_err(AppError::from)?;
    Ok(Json(ClienteResponse::from(cliente)))
}

#[instrument(name = "cliente.get", skip(state), fields(dni = id), err)]
pub async fn get_cliente_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ClienteResponse>, AppError> {
    let cliente = cliente::service::get_by_dni(&state.db, id).await?;
    Ok(Json(ClienteResponse::from(cliente)))
}

#[instrument(name = "cliente.update", skip(state, request), fields(dni = id), err)]
pub async fn update_nombre_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(request): Json<ClienteRequest>,
) -> Result<Json<ClienteResponse>, AppError> {
    let cliente = cliente::service::update_nombre(&state.db, request)
        .await
        .map_err(AppError::from)?;
    Ok(Json(ClienteResponse::from(cliente)))
}

#[instrument(name = "cliente.update_password", skip(state, request), err)]
pub async fn update_password_handler(
    State(state): State<AppState>,
    Json(request): Json<UpdatePasswordRequest>,
) -> Result<(), AppError> {
    let _ = cliente::service::update_password(&state.db, request)
        .await
        .map_err(AppError::from)?;
    Ok(())
}

#[instrument(name = "cliente.update_estado", skip(state, request), err)]
pub async fn update_estado_handler(
    State(state): State<AppState>,
    Json(request): Json<ClienteRequest>,
) -> Result<Json<ClienteResponse>, AppError> {
    let cliente = cliente::service::update_estado(&state.db, request)
        .await
        .map_err(AppError::from)?;
    Ok(Json(ClienteResponse::from(cliente)))
}

#[instrument(name = "cliente.reset_password", skip(state, email), err)]
pub async fn reset_password_handler(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<(), AppError> {
    cliente::service::reset_password(&state.db, &email, &state.mailer)
        .await
        .map_err(AppError::from)?;
    Ok(())
}

#[instrument(name = "cliente.delete", skip(state), fields(dni = request.dni), err)]
pub async fn delete_cliente_handler(
    State(state): State<AppState>,
    Json(request): Json<EliminarClienteRequest>,
) -> Result<impl IntoResponse, AppError> {
    cliente::service::delete(&state.db, request).await?;
    Ok(StatusCode::OK)
}

#[instrument(name = "cliente.list", skip(state), err)]
pub async fn get_clientes_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ClienteResponse>>, AppError> {
    let clientes = cliente::service::get_all(&state.db).await?;
    Ok(Json(
        clientes.into_iter().map(ClienteResponse::from).collect(),
    ))
}
