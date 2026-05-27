use axum::extract::Json;
use axum::extract::State;
use tracing::instrument;

use crate::app_state::AppState;
use crate::dtos::CreateListaEsperaRequest;
use crate::errors::ApiError;

#[instrument(name = "lista_espera.create", skip(state, request), err)]
pub async fn create_lista_espera(
    State(state): State<AppState>,
    Json(request): Json<CreateListaEsperaRequest>,
) -> Result<Json<()>, ApiError> {
    todo!()
}
