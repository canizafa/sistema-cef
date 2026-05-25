use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    app_state::AppState, domain::Empleado, errors::ApiError, repository::EmpleadoRepository,
};

pub async fn create_empleado_handler(
    State(state): State<AppState>,
    Json(body): Json<Empleado>,
) -> Result<Json<Empleado>, ApiError> {
    let empleado = Empleado::from(body);
    empleado.validate_empleado()?;
    EmpleadoRepository::create_empleado(&state.db, &empleado).await?;
    Ok(Json(empleado))
}

pub async fn get_empleado_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Empleado>, ApiError> {
    let empleado = EmpleadoRepository::get_by_dni(&state.db, id).await?;
    Ok(Json(empleado))
}

pub async fn get_empleados_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Empleado>>, ApiError> {
    let empleados = EmpleadoRepository::get_empleados(&state.db).await?;
    Ok(Json(empleados))
}

pub async fn update_empleado_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<Empleado>,
) -> Result<Json<Empleado>, ApiError> {
    let empleado = Empleado::from(body);
    empleado.validate_empleado()?;
    EmpleadoRepository::update_empleado(&state.db, id, &empleado).await?;
    Ok(Json(empleado))
}

pub async fn delete_empleado_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<()>, ApiError> {
    EmpleadoRepository::delete_empleado(&state.db, id).await?;
    Ok(Json(()))
}
