use crate::{
    app::{AppError, AppState},
    lista_espera::cliente_lista_espera::{
        self,
        dto::{ClienteListaEsperaResponse, CreateClienteListaEsperaRequest},
    },
};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn create_cliente_lista_espera_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateClienteListaEsperaRequest>,
) -> Result<Json<ClienteListaEsperaResponse>, AppError> {
    let cliente = cliente_lista_espera::service::create(&state.pool, request).await?;

    Ok(Json(ClienteListaEsperaResponse::from(cliente)))
}

pub async fn get_clientes_lista_espera_handler(
    State(state): State<AppState>,
    Path(id_espera): Path<String>,
) -> Result<Json<Vec<ClienteListaEsperaResponse>>, AppError> {
    let clientes = cliente_lista_espera::service::get_all(&state.pool, &id_espera).await?;

    let response = clientes
        .into_iter()
        .map(ClienteListaEsperaResponse::from)
        .collect();

    Ok(Json(response))
}

pub async fn get_next_cliente_handler(
    State(state): State<AppState>,
    Path(id_espera): Path<String>,
) -> Result<Json<ClienteListaEsperaResponse>, AppError> {
    let cliente = cliente_lista_espera::service::get_next(&state.pool, &id_espera).await?;

    match cliente {
        Some(cliente) => Ok(Json(ClienteListaEsperaResponse::from(cliente))),
        None => Err(AppError::NotFound("lista de espera vacia".into())),
    }
}

pub async fn delete_cliente_lista_espera_handler(
    State(state): State<AppState>,
    Path((id_espera, dni_cliente)): Path<(String, i64)>,
) -> Result<StatusCode, AppError> {
    cliente_lista_espera::service::delete(&state.pool, &id_espera, dni_cliente).await?;

    Ok(StatusCode::OK)
}
