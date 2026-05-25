use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app_state::AppState, domain::Empleado, dtos::EmpleadoResponse, errors::ApiError,
    repository::EmpleadoRepository,
};

pub async fn create_empleado_handler(
    State(state): State<AppState>,
    Json(body): Json<Empleado>,
) -> Result<Json<EmpleadoResponse>, ApiError> {
    let empleado = Empleado::from(body);
    empleado.validate_empleado()?;
    EmpleadoRepository::create_empleado(&state.db, &empleado).await?;
    Ok(Json(EmpleadoResponse::from(empleado)))
}

pub async fn get_empleado_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<EmpleadoResponse>, ApiError> {
    let empleado = EmpleadoRepository::get_by_dni(&state.db, id).await?;
    Ok(Json(EmpleadoResponse::from(empleado)))
}

pub async fn get_empleados_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<EmpleadoResponse>>, ApiError> {
    let empleados = EmpleadoRepository::get_empleados(&state.db).await?;
    Ok(Json(
        empleados.into_iter().map(EmpleadoResponse::from).collect(),
    ))
}

pub async fn update_empleado_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<Empleado>,
) -> Result<Json<EmpleadoResponse>, ApiError> {
    let empleado = Empleado::from(body);
    empleado.validate_empleado()?;
    EmpleadoRepository::update_empleado(&state.db, id, &empleado).await?;
    Ok(Json(EmpleadoResponse::from(empleado)))
}

pub async fn delete_empleado_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    EmpleadoRepository::delete_empleado(&state.db, id).await?;
    Ok(StatusCode::OK.into_response())
}
