use super::*;
use crate::app::*;
use crate::mercado_pago::*;
use axum::{Json, extract::State};
use tracing::instrument;

#[instrument(name = "pago.create", skip(state, payload), fields(monto = payload.monto), err)]
pub async fn crear_pago_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreatePagoRequest>,
) -> Result<Json<PagoResponse>, ApiError> {
    let pago = Pago::from(payload.clone());
    pago.validate_pago()?;
    let mp_response = crear_preferencia(payload.titulo, payload.monto)
        .await
        .map_err(|e| ApiError::MpError(e.to_string()))?;

    PagoRepository::create_pago(&state.db, &pago)
        .await
        .map_err(|e| ApiError::MpError(e.to_string()))?;

    Ok(Json(PagoResponse {
        init_point: mp_response.init_point,
        sandbox_init_point: mp_response.sandbox_init_point,
    }))
}
