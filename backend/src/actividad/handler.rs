use super::{dto::ActividadResponse, dto::CreateActividadRequest, service};
use crate::app::errors::AppError;
use crate::app::state::AppState;
use axum::{Json, extract::Path, extract::State};
use reqwest::StatusCode;
use tracing::instrument;

#[instrument(name = "actividad.create", skip(state, request), err)]
pub async fn create_actividad_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateActividadRequest>,
) -> Result<(StatusCode, Json<ActividadResponse>), AppError> {
    let actividad = service::create(&state.db, request).await?;
    Ok((
        StatusCode::CREATED,
        Json(ActividadResponse::from(actividad)),
    ))
}

#[instrument(name = "actividad.get", skip(state), fields(id = %id), err)]
pub async fn get_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<ActividadResponse>), AppError> {
    let actividad = service::get_by_id(&state.db, &id).await?;
    Ok((StatusCode::OK, Json(ActividadResponse::from(actividad))))
}

#[instrument(name = "actividad.list", skip(state), err)]
pub async fn get_actividades_handler(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<ActividadResponse>>), AppError> {
    let actividades = service::get_all(&state.db).await?;
    Ok((
        StatusCode::OK,
        Json(
            actividades
                .into_iter()
                .map(ActividadResponse::from)
                .collect(),
        ),
    ))
}

#[instrument(name = "actividad.delete", skip(state), fields(id = %id), err)]
pub async fn delete_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let _ = service::delete(&state.db, &id).await?;
    Ok(StatusCode::OK)
}

#[instrument(name = "actividad.update", skip(state, request), fields(id = %id), err)]
pub async fn update_actividad_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CreateActividadRequest>,
) -> Result<(StatusCode, Json<ActividadResponse>), AppError> {
    let actividad = service::update(&state.db, &id, request).await?;
    Ok((StatusCode::OK, Json(ActividadResponse::from(actividad))))
}
