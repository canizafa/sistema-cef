use super::*;
use crate::app::*;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::instrument;

#[instrument(name = "empleado.create", skip(state, body), fields(dni = body.dni), err)]
pub async fn create_empleado_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateEmpleadoRequest>,
) -> Result<Json<EmpleadoResponse>, ApiError> {
    let empleado = Empleado::from(body);
    let existe = EmpleadoRepository::get_by_dni(&state.db, empleado.get_dni())
        .await
        .is_ok();
    if existe {
        return Err(ApiError::EmailAlreadyExists);
    }
    empleado.validate_empleado()?;
    EmpleadoRepository::create_empleado(&state.db, &empleado).await?;
    Ok(Json(EmpleadoResponse::from(empleado)))
}

#[instrument(name = "empleado.get", skip(state), fields(dni = id), err)]
pub async fn get_empleado_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<EmpleadoResponse>, ApiError> {
    let empleado = EmpleadoRepository::get_by_dni(&state.db, id).await?;
    Ok(Json(EmpleadoResponse::from(empleado)))
}

#[instrument(name = "empleado.list", skip(state), err)]
pub async fn get_empleados_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<EmpleadoResponse>>, ApiError> {
    let empleados = EmpleadoRepository::get_empleados(&state.db).await?;
    Ok(Json(
        empleados.into_iter().map(EmpleadoResponse::from).collect(),
    ))
}
#[instrument(name = "empleado.update", skip(state, body), fields(dni = id), err)]
#[axum::debug_handler]
pub async fn update_empleado_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateEmpleadoRequest>,
) -> Result<Json<EmpleadoResponse>, ApiError> {
    let empleado = Empleado::from(body);
    empleado.validate_empleado()?;
    EmpleadoRepository::update_empleado(&state.db, id, &empleado).await?;
    Ok(Json(EmpleadoResponse::from(empleado)))
}

#[instrument(name = "empleado.delete", skip(state), fields(dni = id), err)]
pub async fn delete_empleado_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    EmpleadoRepository::delete_empleado(&state.db, id).await?;
    Ok(StatusCode::OK.into_response())
}
