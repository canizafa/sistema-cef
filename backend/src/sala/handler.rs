use crate::{
    app::{errors::AppError, state::AppState},
    sala::{
        self,
        dto::{CreateSalaRequest, SalaResponse},
    },
};
use axum::{Json, extract::Path, extract::State};
use tracing::instrument;

#[instrument(name = "sala.create", skip(state, request), err)]
pub async fn create_sala_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateSalaRequest>,
) -> Result<Json<SalaResponse>, AppError> {
    let sala = sala::service::create(&state.db, request).await?;
    Ok(Json(SalaResponse::from(sala)))
}

#[instrument(name = "sala.get", skip(state), fields(id = %id), err)]
pub async fn get_sala_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<SalaResponse>, AppError> {
    let sala = sala::service::get_by_id(&state.db, &id).await?;
    Ok(Json(SalaResponse::from(sala)))
}

#[instrument(name = "sala.list", skip(state), err)]
pub async fn get_salas_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<SalaResponse>>, AppError> {
    let salas = sala::service::get_all(&state.db).await?;
    Ok(Json(salas.into_iter().map(SalaResponse::from).collect()))
}
