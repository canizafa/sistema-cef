use axum::{Json, extract::State};

use crate::{
    app_state::AppState,
    auth::generar_token,
    domain::{Cliente, Empleado, Rol},
    dtos::{
        AuthResponse, CreateClienteRequest, CreateEmpleadoRequest, LoginRequest,
        ResetPasswordRequest,
    },
    errors::ApiError,
    repository::{ClienteRepository, EmpleadoRepository},
};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let (dni, email, password_hash, rol) = match body.rol {
        Rol::Cliente => {
            let cliente = ClienteRepository::find_by_email(&state.db, &body.email).await?;
            (
                cliente.get_dni(),
                cliente.get_email(),
                cliente.get_password_hash(),
                Rol::Cliente,
            )
        }
        Rol::Profesor => {
            let profesor = EmpleadoRepository::get_by_email(&state.db, &body.email).await?;
            (
                profesor.get_dni(),
                profesor.get_mail(),
                profesor.get_password_hash(),
                Rol::Profesor,
            )
        }
        Rol::Empleado => {
            let empleado = EmpleadoRepository::get_by_email(&state.db, &body.email).await?;
            (
                empleado.get_dni(),
                empleado.get_mail(),
                empleado.get_password_hash(),
                Rol::Empleado,
            )
        }
        Rol::Duenio => {
            let duenio = EmpleadoRepository::get_by_email(&state.db, &body.email).await?;
            (
                duenio.get_dni(),
                duenio.get_mail(),
                duenio.get_password_hash(),
                Rol::Duenio,
            )
        }
        _ => return Err(ApiError::Unauthorized),
    };
    if password_hash != body.password {
        return Err(ApiError::Unauthorized);
    }

    let token =
        generar_token(&dni, rol.clone(), &state.jwt_secret).map_err(|_| ApiError::JwtTokenError)?;

    Ok(Json(AuthResponse {
        dni: dni.to_string(),
        email: email.to_string(),
        access_token: token,
        rol,
    }))
}

pub async fn register_cliente_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateClienteRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let cliente = Cliente::from(body);
    ClienteRepository::create_cliente(&state.db, &cliente).await?;

    let token = generar_token(
        &cliente.get_dni(),
        cliente.get_rol().clone(),
        &state.jwt_secret,
    )
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(AuthResponse {
        dni: cliente.get_dni(),
        email: cliente.get_email(),
        access_token: token,
        rol: cliente.get_rol().clone(),
    }))
}

pub async fn reset_password_cliente_handler(
    State(state): State<AppState>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let cliente = ClienteRepository::find_by_email(&state.db, &body.email).await?;

    ClienteRepository::reset_password(&state.db, &cliente.get_dni(), &body.password).await?;

    let token = generar_token(
        &cliente.get_dni(),
        cliente.get_rol().clone(),
        &state.jwt_secret,
    )
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(AuthResponse {
        dni: cliente.get_dni(),
        email: cliente.get_email(),
        access_token: token,
        rol: cliente.get_rol().clone(),
    }))
}

pub async fn register_empleado_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateEmpleadoRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let empleado = Empleado::from(body);
    EmpleadoRepository::create_empleado(&state.db, &empleado).await?;

    let token = generar_token(
        &empleado.get_dni(),
        empleado.get_rol().clone(),
        &state.jwt_secret,
    )
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(AuthResponse {
        dni: empleado.get_dni(),
        email: empleado.get_mail(),
        access_token: token,
        rol: empleado.get_rol().clone(),
    }))
}

pub async fn login_empleado_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let empleado = EmpleadoRepository::get_by_email(&state.db, &body.email).await?;

    let token = generar_token(
        &empleado.get_dni(),
        empleado.get_rol().clone(),
        &state.jwt_secret,
    )
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(AuthResponse {
        dni: empleado.get_dni(),
        email: empleado.get_mail(),
        access_token: token,
        rol: empleado.get_rol().clone(),
    }))
}

pub async fn reset_password_empleado_handler(
    State(state): State<AppState>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let mut empleado = EmpleadoRepository::get_by_email(&state.db, &body.email).await?;

    let token = generar_token(
        &empleado.get_dni(),
        empleado.get_rol().clone(),
        &state.jwt_secret,
    )
    .map_err(|_| ApiError::InternalServerError)?;

    empleado.update_password(&body.password)?;

    Ok(Json(AuthResponse {
        dni: empleado.get_dni(),
        email: empleado.get_mail(),
        access_token: token,
        rol: empleado.get_rol().clone(),
    }))
}
