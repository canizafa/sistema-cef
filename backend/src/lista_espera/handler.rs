use super::dto::{CreateListaEsperaRequest, ListaEsperaResponse};
use crate::app::errors::AppError;
use crate::app::state::AppState;
use crate::lista_espera;
use axum::Json;
use axum::extract::{Path, State};
use reqwest::StatusCode;
use tracing::instrument;

#[instrument(name = "lista_espera.create", skip(state, request), err)]
pub async fn create_lista_espera_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateListaEsperaRequest>,
) -> Result<Json<ListaEsperaResponse>, AppError> {
    let lista = lista_espera::service::create(&state.db, request).await?;

    Ok(Json(lista.into()))
}

#[instrument(name = "lista_espera.get", skip(state), fields(id = %id), err)]
pub async fn get_lista_espera_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ListaEsperaResponse>, AppError> {
    let lista = lista_espera::service::get_by_id(&state.db, &id).await?;
    Ok(Json(lista.into()))
}

#[instrument(name = "lista_espera.list", skip(state), err)]
pub async fn get_all_lista_espera_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ListaEsperaResponse>>, AppError> {
    let listas = lista_espera::service::get_all(&state.db).await?;

    Ok(Json(
        listas.into_iter().map(ListaEsperaResponse::from).collect(),
    ))
}

#[instrument(name = "lista_espera.delete", skip(state), fields(id = %id), err)]
pub async fn delete_lista_espera_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    lista_espera::service::delete(&state.db, &id).await?;
    Ok(StatusCode::OK)
}
