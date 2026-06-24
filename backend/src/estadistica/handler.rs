use super::dto::{
    ClaseMasCanceladaResponse, ClaseMasConcurridaResponse, EstadisticaRequest, RecaudacionResponse,
};
use crate::{
    app::{errors::AppError, state::AppState},
    estadistica,
};
use axum::{
    Json,
    extract::{Query, State},
};
use tracing::instrument;

#[instrument(name = "estadistica.concurrencia", skip(state), err)]
pub async fn clase_mas_concurrida_handler(
    State(state): State<AppState>,
    Query(request): Query<EstadisticaRequest>,
) -> Result<Json<ClaseMasConcurridaResponse>, AppError> {
    let estadistica = estadistica::service::obtener_clase_mas_concurrida(
        &state.db,
        request.fecha_desde,
        request.fecha_hasta,
    )
    .await?;

    Ok(Json(estadistica.into()))
}
#[instrument(name = "estadistica.cancelaciones", skip(state), err)]
pub async fn clase_mas_cancelada_handler(
    State(state): State<AppState>,
    Query(request): Query<EstadisticaRequest>,
) -> Result<Json<ClaseMasCanceladaResponse>, AppError> {
    let estadistica = estadistica::service::obtener_clase_mas_cancelada(
        &state.db,
        request.fecha_desde,
        request.fecha_hasta,
    )
    .await?;

    Ok(Json(estadistica.into()))
}
#[instrument(name = "estadistica.recaudacion", skip(state), err)]
pub async fn recaudacion_handler(
    State(state): State<AppState>,
    Query(request): Query<EstadisticaRequest>,
) -> Result<Json<RecaudacionResponse>, AppError> {
    let recaudacion = estadistica::service::obtener_recaudacion(
        &state.db,
        request.fecha_desde,
        request.fecha_hasta,
    )
    .await?;

    Ok(Json(recaudacion.into()))
}
