use super::*;
use crate::app::*;
use axum::extract::Json;
use axum::extract::State;

pub async fn create_lista_espera(
    State(state): State<AppState>,
    Json(request): Json<CreateListaEsperaRequest>,
) -> Result<Json<()>, ApiError> {
    todo!()
}
