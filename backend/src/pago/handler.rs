use crate::app::{errors::AppError, state::AppState};
use crate::mercado_pago::*;
use crate::pago::dto::{CreatePagoRequest, PagoResponse};
use axum::{Json, extract::State};
use tracing::instrument;

#[instrument(name = "pago.create", skip(_state, payload), fields(monto = payload.monto), err)]
pub async fn crear_pago_handler(
    State(_state): State<AppState>,
    Json(payload): Json<CreatePagoRequest>,
) -> Result<Json<PagoResponse>, AppError> {
    let mp_response = crear_preferencia(payload.titulo, payload.monto)
        .await
        .map_err(AppError::from)?;

    Ok(Json(PagoResponse {
        init_point: mp_response.init_point,
        sandbox_init_point: mp_response.sandbox_init_point,
    }))
}
