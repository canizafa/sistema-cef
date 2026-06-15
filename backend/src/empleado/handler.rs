use crate::{
    app::{errors::AppError, state::AppState},
    empleado::{
        self,
        dto::{
            CreateEmpleadoRequest, EliminarEmpleadoRequest, EmpleadoResponse, UpdateEmpleadoRequest,
        },
    },
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::instrument;

#[instrument(name = "empleado.create", skip(state, request), fields(dni = request.dni), err)]
pub async fn create_empleado_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateEmpleadoRequest>,
) -> Result<Json<EmpleadoResponse>, AppError> {
    let empleado = empleado::service::create(&state.db, request)
        .await
        .map_err(AppError::from)?;
    Ok(Json(EmpleadoResponse::from(empleado)))
}

#[instrument(name = "empleado.get", skip(state), fields(dni = id), err)]
pub async fn get_empleado_by_dni_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<EmpleadoResponse>, AppError> {
    let empleado = empleado::service::get_by_dni(&state.db, id)
        .await
        .map_err(AppError::from)?;
    Ok(Json(EmpleadoResponse::from(empleado)))
}
#[instrument(name = "empleado.get", skip(state), fields(email = id), err)]
pub async fn get_empleado_by_email_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<EmpleadoResponse>, AppError> {
    let empleado = empleado::service::get_by_email(&state.db, &id)
        .await
        .map_err(AppError::from)?;
    Ok(Json(EmpleadoResponse::from(empleado)))
}

#[instrument(name = "empleado.list", skip(state), err)]
pub async fn get_empleados_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<EmpleadoResponse>>, AppError> {
    let empleados = empleado::service::get_all(&state.db)
        .await
        .map_err(AppError::from)?;
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
) -> Result<Json<EmpleadoResponse>, AppError> {
    let empleado = empleado::service::update(&state.db, id, body)
        .await
        .map_err(AppError::from)?;
    Ok(Json(EmpleadoResponse::from(empleado)))
}

#[instrument(name = "empleado.delete", skip(state), fields(dni = id), err)]
pub async fn delete_empleado_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(request): Json<EliminarEmpleadoRequest>,
) -> Result<impl IntoResponse, AppError> {
    empleado::service::delete(&state.db, id, &request.motivo_eliminacion.to_string()).await?;
    Ok(StatusCode::OK.into_response())
}
