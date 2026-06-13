use super::*;
use crate::app::{ApiError, AppState};
use axum::{
    Json,
    extract::{Path, State},
};
use tracing::instrument;

#[instrument(name = "profesor.create", skip(state, request), fields(dni = request.dni), err)]
pub async fn create_profesor_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateProfesorRequest>,
) -> Result<Json<ProfesorResponse>, ApiError> {
    let profesor = Profesor::from(request);
    let existe = ProfesorRepository::get_profesor_by_dni(&state.db, profesor.get_dni())
        .await
        .is_ok();
    if existe {
        return Err(ApiError::EmailAlreadyExists);
    }
    profesor.validate_profesor()?;
    let profesor = ProfesorRepository::create_profesor(&state.db, &profesor).await?;
    Ok(Json(ProfesorResponse::from(profesor)))
}

#[instrument(name = "profesor.get", skip(state), fields(dni = dni), err)]
pub async fn get_profesor_by_dni_handler(
    State(state): State<AppState>,
    Path(dni): Path<i64>,
) -> Result<Json<ProfesorResponse>, ApiError> {
    let profesor = ProfesorRepository::get_profesor_by_dni(&state.db, dni).await?;
    Ok(Json(ProfesorResponse::from(profesor)))
}

#[instrument(name = "profesor.list", skip(state), err)]
pub async fn get_profesores_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProfesorResponse>>, ApiError> {
    let profesores = ProfesorRepository::get_all(&state.db).await?;
    Ok(Json(
        profesores
            .into_iter()
            .map(|p| ProfesorResponse::from(p))
            .collect(),
    ))
}

#[instrument(name = "profesor.update", skip(state, request), fields(dni = dni), err)]
pub async fn update_profesor_handler(
    State(state): State<AppState>,
    Path(dni): Path<i64>,
    Json(request): Json<CreateProfesorRequest>,
) -> Result<Json<ProfesorResponse>, ApiError> {
    let profesor = Profesor::from(request);
    profesor.validate_profesor()?;
    let profesor = ProfesorRepository::update_profesor(&state.db, dni, &profesor).await?;
    Ok(Json(ProfesorResponse::from(profesor)))
}

#[instrument(name = "profesor.delete", skip(state), fields(dni = dni), err)]
pub async fn delete_profesor_handler(
    State(state): State<AppState>,
    Path(dni): Path<i64>,
) -> Result<Json<()>, ApiError> {
    ProfesorRepository::delete_profesor(&state.db, dni).await?;
    Ok(Json(()))
}
