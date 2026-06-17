use super::dto::{ClaseResponse, CreateClaseRequest};
use crate::{
    app::{errors::AppError, state::AppState},
    clase::{self, dto::UpdateClaseRequest},
};

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use tracing::instrument;

#[instrument(name = "clase.create", skip(state, request), err)]
pub async fn create_clase_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateClaseRequest>,
) -> Result<Json<ClaseResponse>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let clase = clase::service::create(&state.db, request, &id).await?;
    Ok(Json(ClaseResponse::from(clase)))
}

#[instrument(name = "clase.get", skip(state), fields(id = %id), err)]
pub async fn get_clase_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ClaseResponse>, AppError> {
    let clase = clase::service::get_by_id(&state.db, &id).await?;
    Ok(Json(ClaseResponse::from(clase)))
}

#[axum::debug_handler]
#[instrument(name = "clase.update", skip(state, request), fields(id = %id), err)]
pub async fn update_clase_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateClaseRequest>,
) -> Result<Json<ClaseResponse>, AppError> {
    let clase = clase::service::update(&state.db, &id, request).await?;
    Ok(Json(ClaseResponse::from(clase)))
}

#[instrument(name = "clase.delete", skip(state), fields(id = %id), err)]
pub async fn delete_clase_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    clase::service::delete(&state.db, &id).await?;
    Ok(StatusCode::OK)
}

#[instrument(name = "clase.list", skip(state), err)]
pub async fn get_clases_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ClaseResponse>>, AppError> {
    let clases = clase::service::get_all(&state.db).await?;
    Ok(Json(clases.into_iter().map(ClaseResponse::from).collect()))
}
