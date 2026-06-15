use crate::{
    app::{errors::AppError, state::AppState},
    profesor::{
        self,
        dto::{CreateProfesorRequest, EliminarProfesorRequest, ProfesorResponse},
    },
};
use axum::{
    Json,
    extract::{Path, State},
};
use tracing::instrument;

#[instrument(name = "profesor.create", skip(state, request), fields(dni = request.dni), err)]
pub async fn create_profesor_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateProfesorRequest>,
) -> Result<Json<ProfesorResponse>, AppError> {
    let profesor = profesor::service::create(&state.db, request)
        .await
        .map_err(AppError::from)?;
    Ok(Json(ProfesorResponse::from(profesor)))
}

#[instrument(name = "profesor.get", skip(state), fields(dni = dni), err)]
pub async fn get_profesor_by_dni_handler(
    State(state): State<AppState>,
    Path(dni): Path<i64>,
) -> Result<Json<ProfesorResponse>, AppError> {
    let profesor = profesor::service::get_by_dni(&state.db, dni)
        .await
        .map_err(AppError::from)?;
    Ok(Json(ProfesorResponse::from(profesor)))
}

#[instrument(name = "profesor.list", skip(state), err)]
pub async fn get_profesores_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProfesorResponse>>, AppError> {
    let profesores = profesor::service::get_all(&state.db)
        .await
        .map_err(AppError::from)?;
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
) -> Result<Json<ProfesorResponse>, AppError> {
    let profesor = profesor::service::update(&state.db, request)
        .await
        .map_err(AppError::from)?;
    Ok(Json(ProfesorResponse::from(profesor)))
}

#[instrument(name = "profesor.delete", skip(state), fields(dni = dni), err)]
pub async fn delete_profesor_handler(
    State(state): State<AppState>,
    Path(dni): Path<i64>,
    Json(request): Json<EliminarProfesorRequest>,
) -> Result<Json<()>, AppError> {
    profesor::service::delete(&state.db, dni, &request.motivo_eliminacion)
        .await
        .map_err(AppError::from)?;
    Ok(Json(()))
}
