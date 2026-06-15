use std::env;

use sqlx::SqlitePool;

use crate::{
    app::{errors::AppError, mailer::Mailer, rol::Rol},
    auth::{
        dto::{AuthResponse, LoginRequest},
        jwt,
    },
    cliente, empleado,
};

pub async fn login(db: &SqlitePool, request: LoginRequest) -> Result<AuthResponse, AppError> {
    if let Ok(cliente) =
        cliente::service::login_cliente(db, &request.email, &request.password).await
    {
        let jwt_token = jwt::generar_token(
            cliente.get_dni(),
            Rol::Cliente,
            &env::var("JWT_SECRET").unwrap_or_default(),
        )?;
        Ok(AuthResponse {
            dni: cliente.get_dni(),
            email: cliente.get_mail(),
            access_token: jwt_token,
            rol: Rol::Cliente,
        })
    } else if let Ok(empleado) =
        empleado::service::login_empleado(db, &request.email, &request.password).await
    {
        let jwt_token = jwt::generar_token(
            empleado.get_dni(),
            Rol::Empleado,
            &env::var("JWT_SECRET").unwrap_or_default(),
        )?;
        Ok(AuthResponse {
            dni: empleado.get_dni(),
            email: empleado.get_mail(),
            access_token: jwt_token,
            rol: Rol::Empleado,
        })
    } else {
        return Err(AppError::InvalidCredentials);
    }
}

pub async fn reset_password(db: &SqlitePool, email: &str, mailer: &Mailer) -> Result<(), AppError> {
    //Nos fijamos si es un cliente o empleado
    let cliente = cliente::service::get_by_email(db, email).await;
    let empleado = empleado::service::get_by_email(db, email).await;
    if cliente.is_ok() {
        cliente::service::reset_password(db, email, mailer).await?;
        Ok(())
    } else if empleado.is_ok() {
        empleado::service::reset_password(db, email, mailer).await?;
        Ok(())
    } else {
        return Err(AppError::NotFound("Usuario no encontrado".to_string()));
    }
}

pub async fn change_password(
    db: &SqlitePool,
    dni: i64,
    old_password: &str,
    new_password: &str,
) -> Result<(), AppError> {
    let usuario_cliente = cliente::service::get_by_dni(db, dni).await;
    let usuario_empleado = empleado::service::get_by_dni(db, dni).await;
    if usuario_cliente.is_ok() {
        cliente::service::change_password(db, dni, old_password, new_password).await?;
        Ok(())
    } else if usuario_empleado.is_ok() {
        empleado::service::change_password(db, dni, old_password, new_password).await?;
        Ok(())
    } else {
        return Err(AppError::NotFound("Usuario no encontrado".to_string()));
    }
}
