use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::instrument;

use crate::{
    app_state::AppState,
    domain::Cliente,
    dtos::{ClienteResponse, CreateClienteRequest, UpdateClienteRequest},
    errors::ApiError,
    repository::ClienteRepository,
};

#[instrument(name = "cliente.create", skip(state, request), fields(dni = request.dni), err)]
pub async fn create_cliente_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateClienteRequest>,
) -> Result<Json<ClienteResponse>, ApiError> {
    let cliente = Cliente::from(request);
    let existe = ClienteRepository::get_by_dni(&state.db, cliente.get_dni())
        .await
        .is_ok();
    if existe {
        return Err(ApiError::EmailAlreadyExists);
    }
    cliente.validate_cliente()?;
    ClienteRepository::create_cliente(&state.db, &cliente).await?;
    Ok(Json(ClienteResponse::from(cliente)))
}

#[instrument(name = "cliente.get", skip(state), fields(dni = id), err)]
pub async fn get_cliente_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ClienteResponse>, ApiError> {
    let cliente = ClienteRepository::get_by_dni(&state.db, id).await?;
    Ok(Json(ClienteResponse::from(cliente)))
}

#[instrument(name = "cliente.update", skip(state, request), fields(dni = id), err)]
pub async fn update_cliente_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateClienteRequest>,
) -> Result<Json<ClienteResponse>, ApiError> {
    let cliente = Cliente::from(request);
    cliente.validate_cliente()?;
    ClienteRepository::update_cliente(&state.db, id, &cliente).await?;
    Ok(Json(ClienteResponse::from(cliente)))
}

#[instrument(name = "cliente.delete", skip(state), fields(dni = id), err)]
pub async fn delete_cliente_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    ClienteRepository::delete_cliente(&state.db, id).await?;
    Ok(StatusCode::OK)
}

#[instrument(name = "cliente.list", skip(state), err)]
pub async fn get_clientes_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ClienteResponse>>, ApiError> {
    let clientes = ClienteRepository::list_clientes(&state.db).await?;
    Ok(Json(
        clientes.into_iter().map(ClienteResponse::from).collect(),
    ))
}
