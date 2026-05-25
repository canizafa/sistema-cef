use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    app_state::AppState,
    auth::{generar_token, generate_random_password},
    domain::{Cliente, Empleado},
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
    let usuario_cliente = ClienteRepository::get_by_email(&state.db, &body.email).await;
    let usuario_empleado = EmpleadoRepository::get_by_email(&state.db, &body.email).await;

    if usuario_cliente.is_err() {
        if usuario_empleado.is_err() {
            return Err(ApiError::UserNotFound);
        }
        let usuario = usuario_empleado.unwrap();
        let token = generar_token(usuario.get_dni(), body.rol.clone(), &state.jwt_secret)
            .map_err(|_| ApiError::JwtTokenError)?;
        Ok(Json(AuthResponse {
            dni: usuario.get_dni().to_string(),
            email: usuario.get_email().to_string(),
            access_token: token,
            rol: body.rol,
        }))
    } else {
        let usuario = usuario_cliente.unwrap();
        let token = generar_token(usuario.get_dni(), body.rol.clone(), &state.jwt_secret)
            .map_err(|_| ApiError::JwtTokenError)?;
        Ok(Json(AuthResponse {
            dni: usuario.get_dni().to_string(),
            email: usuario.get_email().to_string(),
            access_token: token,
            rol: body.rol,
        }))
    }
}

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
    )
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(AuthResponse {
        dni: cliente.get_dni().to_string(),
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
    empleado.validate_empleado()?;

    EmpleadoRepository::create_empleado(&state.db, &empleado).await?;

    let token = generar_token(
        empleado.get_dni(),
        empleado.get_rol().clone(),
        &state.jwt_secret,
    )
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(AuthResponse {
        dni: empleado.get_dni().to_string(),
        email: empleado.get_email(),
        access_token: token,
        rol: empleado.get_rol().clone(),
    }))
}

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
        let new_password = generate_random_password(empleado.get_dni() as usize);
        empleado.update_password(&new_password)?;
        EmpleadoRepository::update_empleado(&state.db, empleado.get_dni(), &empleado).await?;

        state
            .mailer
            .send_new_password(&body.email, &new_password)
            .await
            .map_err(|e| ApiError::InternalServerError)?;

        Ok((
            StatusCode::OK,
            "Contraseña cambiada exitosamente".into_response(),
        ))
    } else {
        let mut cliente = usuario_cliente.unwrap();

        let new_password = generate_random_password(cliente.get_dni() as usize);
        cliente.update_password(&new_password)?;
        ClienteRepository::update_cliente(&state.db, cliente.get_dni(), &cliente).await?;

        state
            .mailer
            .send_new_password(&body.email, &new_password)
            .await
            .map_err(|e| ApiError::InternalServerError)?;

        Ok((
            StatusCode::OK,
            "Contraseña cambiada exitosamente".into_response(),
        ))
    }
}
