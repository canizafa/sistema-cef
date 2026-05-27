use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::instrument;

use crate::{
    app_state::AppState,
    auth::{generar_token, generate_random_password, password::hash_password},
    domain::{Cliente, Empleado},
    dtos::{
        AuthResponse, CreateChangePasswordRequest, CreateClienteRequest, CreateEmpleadoRequest,
        LoginRequest, ResetPasswordRequest,
    },
    errors::ApiError,
    repository::{ClienteRepository, EmpleadoRepository},
};

#[instrument(name = "auth.login", skip(state, body), fields(email = %body.email), err)]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let usuario_cliente = ClienteRepository::get_by_email(&state.db, &body.email).await;
    let usuario_empleado = EmpleadoRepository::get_by_email(&state.db, &body.email).await;

    if usuario_cliente.is_err() {
        if usuario_empleado.is_err() {
            return Err(ApiError::UserNotFound);
        }
        let usuario = usuario_empleado.unwrap();
        let rol = usuario.get_rol();
        let token = generar_token(usuario.get_dni(), rol.clone(), &state.jwt_secret)?;
        Ok(Json(AuthResponse {
            dni: usuario.get_dni().to_string(),
            email: usuario.get_email().to_string(),
            access_token: token,
            rol,
        }))
    } else {
        let usuario = usuario_cliente.unwrap();
        let rol = usuario.get_rol();
        let token = generar_token(usuario.get_dni(), rol.clone(), &state.jwt_secret)?;
        Ok(Json(AuthResponse {
            dni: usuario.get_dni().to_string(),
            email: usuario.get_email().to_string(),
            access_token: token,
            rol,
        }))
    }
}

#[instrument(name = "auth.register_cliente", skip(state, body), fields(dni = body.dni), err)]
pub async fn register_cliente_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateClienteRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let cliente = Cliente::from(body);
    cliente.validate_cliente()?;

    ClienteRepository::create_cliente(&state.db, &cliente).await?;

    let token = generar_token(
        cliente.get_dni(),
        cliente.get_rol().clone(),
        &state.jwt_secret,
    )?;

    Ok(Json(AuthResponse {
        dni: cliente.get_dni().to_string(),
        email: cliente.get_email(),
        access_token: token,
        rol: cliente.get_rol(),
    }))
}

#[instrument(name = "auth.register_empleado", skip(state, body), fields(dni = body.dni), err)]
pub async fn register_empleado_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateEmpleadoRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let empleado = Empleado::from(body);
    empleado.validate_empleado()?;

    EmpleadoRepository::create_empleado(&state.db, &empleado).await?;

    let token = generar_token(
        empleado.get_dni(),
        empleado.get_rol().clone(),
        &state.jwt_secret,
    )?;

    Ok(Json(AuthResponse {
        dni: empleado.get_dni().to_string(),
        email: empleado.get_email(),
        access_token: token,
        rol: empleado.get_rol(),
    }))
}

#[instrument(name = "auth.reset_password", skip(state, body), fields(email = %body.email), err)]
pub async fn reset_password_handler(
    State(state): State<AppState>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let usuario_cliente = ClienteRepository::get_by_email(&state.db, &body.email).await;
    let usuario_empleado = EmpleadoRepository::get_by_email(&state.db, &body.email).await;
    if usuario_cliente.is_err() {
        if usuario_empleado.is_err() {
            return Err(ApiError::UserNotFound);
        }
        let mut empleado = usuario_empleado.unwrap();
        let new_password = generate_random_password();
        let hashed_password = hash_password(&new_password)?;

        empleado.update_password(&hashed_password)?;
        EmpleadoRepository::update_empleado(&state.db, empleado.get_dni(), &empleado).await?;

        state
            .mailer
            .send_new_password(&body.email, &new_password)
            .await?;

        Ok((
            StatusCode::OK,
            "Contraseña cambiada exitosamente".into_response(),
        ))
    } else {
        let mut cliente = usuario_cliente.unwrap();

        let new_password = generate_random_password();
        let hashed_password = hash_password(&new_password)?;
        cliente.update_password(&hashed_password)?;
        ClienteRepository::update_cliente(&state.db, cliente.get_dni(), &cliente).await?;

        state
            .mailer
            .send_new_password(&body.email, &new_password)
            .await?;

        Ok((
            StatusCode::OK,
            "Contraseña cambiada exitosamente".into_response(),
        ))
    }
}

#[instrument(name = "auth.change_password", skip(state, body), fields(dni = dni), err)]
pub async fn change_password_handler(
    State(state): State<AppState>,
    Path(dni): Path<i64>,
    Json(body): Json<CreateChangePasswordRequest>,
) -> Result<StatusCode, ApiError> {
    let usuario_cliente = ClienteRepository::get_by_dni(&state.db, dni).await;
    let usuario_empleado = EmpleadoRepository::get_by_dni(&state.db, dni).await;
    if usuario_cliente.is_ok() {
        let mut cliente = usuario_cliente.unwrap();
        if !cliente.get_password_hash().eq(&body.old_password) {
            return Err(ApiError::InvalidCredentials);
        }
        let new_password = hash_password(&body.new_password)?;
        cliente.update_password(&new_password)?;

        ClienteRepository::update_cliente(&state.db, dni, &cliente).await?;

        Ok(StatusCode::OK)
    } else if usuario_empleado.is_ok() {
        let mut empleado = usuario_empleado.unwrap();
        if !empleado.get_password_hash().eq(&body.old_password) {
            return Err(ApiError::InvalidCredentials);
        }
        let new_password_hash = hash_password(&body.new_password)?;
        empleado.update_password(&new_password_hash)?;
        EmpleadoRepository::update_empleado(&state.db, dni, &empleado).await?;

        Ok(StatusCode::OK)
    } else {
        return Err(ApiError::NotFound);
    }
}
