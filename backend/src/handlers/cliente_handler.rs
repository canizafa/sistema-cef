use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app_state::AppState, domain::Cliente, dtos::CreateClienteRequest, errors::ApiError,
    repository::ClienteRepository,
};

pub async fn create_cliente_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateClienteRequest>,
) -> Result<Json<Cliente>, ApiError> {
    let cliente = Cliente::from(request);
    cliente.validate_cliente()?;
    ClienteRepository::create_cliente(&state.db, &cliente).await?;
    Ok(Json(cliente))
}

pub async fn get_cliente_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Cliente>, ApiError> {
    let cliente = ClienteRepository::get_by_dni(&state.db, id).await?;
    Ok(Json(cliente))
}

pub async fn update_cliente_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(request): Json<CreateClienteRequest>,
) -> Result<Json<Cliente>, ApiError> {
    let cliente = Cliente::from(request);
    cliente.validate_cliente()?;
    ClienteRepository::update_cliente(&state.db, id, &cliente).await?;
    Ok(Json(cliente))
}

pub async fn delete_cliente_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    ClienteRepository::delete_cliente(&state.db, id).await?;
    Ok(StatusCode::OK)
}

pub async fn get_clientes_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Cliente>>, ApiError> {
    let clientes = ClienteRepository::list_clientes(&state.db).await?;
    Ok(Json(clientes))
}
