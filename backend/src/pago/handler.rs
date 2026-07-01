use crate::app::{errors::AppError, state::AppState};
use crate::mercado_pago::*;
use crate::pago::dto::{CreatePagoRequest, PagoResponse};
use crate::usuarios::cliente;
use axum::{Json, extract::State};
use tracing::instrument;

#[instrument(name = "pago.create", skip(state, payload), fields(monto = payload.monto), err)]
pub async fn crear_pago_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreatePagoRequest>,
) -> Result<Json<PagoResponse>, AppError> {
    let mut cliente = cliente::service::get_by_dni(&state.db, payload.dni).await?;

    let mp_response = crear_preferencia(
        payload.titulo,
        payload.monto - cliente.0.consumir_creditos(payload.monto as i64) as f64,
    )
    .await
    .map_err(AppError::from)?;

    cliente::service::update_creditos_y_cancelaciones(&state.db, &cliente.0).await?;

    Ok(Json(PagoResponse {
        init_point: mp_response.init_point,
        sandbox_init_point: mp_response.sandbox_init_point,
    }))
}
