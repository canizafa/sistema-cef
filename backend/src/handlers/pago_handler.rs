use crate::{
    app_state::AppState,
    domain::pago::Pago,
    dtos::{CreatePagoRequest, PagoResponse},
    errors::ApiError,
    repository::PagoRepository,
    services::mercado_pago_service,
};
use axum::{Json, extract::State};
use tracing::instrument;

#[instrument(name = "pago.create", skip(state, payload), fields(monto = payload.monto), err)]
pub async fn crear_pago_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreatePagoRequest>,
) -> Result<Json<PagoResponse>, ApiError> {
    let pago = Pago::from(payload.clone());
    pago.validate_pago()?;
    let mp_response = mercado_pago_service::crear_preferencia(payload.titulo, payload.monto)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    PagoRepository::create_pago(&state.db, &pago)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(PagoResponse {
        init_point: mp_response.init_point,
        sandbox_init_point: mp_response.sandbox_init_point,
    }))
}
