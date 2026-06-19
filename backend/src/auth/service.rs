use std::env;

use sqlx::SqlitePool;

use crate::{
    app::{errors::AppError, mailer::Mailer},
    auth::{
        dto::{AuthResponse, LoginRequest},
        jwt,
    },
    usuarios::{cliente, empleado},
};

pub async fn login(db: &SqlitePool, request: LoginRequest) -> Result<AuthResponse, AppError> {
    if let Ok(cliente) =
        cliente::service::login_cliente(db, &request.email, &request.password).await
    {
        let jwt_token = jwt::generar_token(
            cliente.get_dni(),
            cliente.get_rol().to_string(),
            cliente.get_estado().to_string(),
            &env::var("JWT_SECRET").unwrap_or_default(),
        )?;
        return Ok(AuthResponse {
            dni: cliente.get_dni(),
            email: cliente.get_mail(),
            access_token: jwt_token,
            rol: "cliente".to_string(),
            estado: cliente.get_estado().to_string(),
        });
    }
    if let Ok(empleado) =
        empleado::service::login_empleado(db, &request.email, &request.password).await
    {
        let jwt_token = jwt::generar_token(
            empleado.get_dni(),
            empleado.get_rol().to_string(),
            empleado.get_estado().to_string(),
            &env::var("JWT_SECRET").unwrap_or_default(),
        )?;
        Ok(AuthResponse {
            dni: empleado.get_dni(),
            email: empleado.get_mail(),
            access_token: jwt_token,
            rol: empleado.get_rol().to_string(),
            estado: empleado.get_estado().to_string(),
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
