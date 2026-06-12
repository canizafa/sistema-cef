use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Hubo un error en la base de datos")]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    Api(#[from] ApiError),
    #[error("Hubo un error interno del servidor")]
    InternalServerError(String),
    #[error("Hubo un error en la migración de la base de datos")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[error("Variable de entorno no encontrada")]
    EnvironmentVariableNotFound,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("no se pudo enviar el correo (SMTP): {0}")]
    SmtpError(#[from] lettre::transport::smtp::Error),
    #[error("dirección de correo inválida: {0}")]
    AddressError(#[from] lettre::address::AddressError),
    #[error("no se pudo armar el mensaje de correo: {0}")]
    MessageError(#[from] lettre::error::Error),
    #[error("asistencia inválida")]
    InvalidAsistencia,
    #[error("error de Mercado Pago: {0}")]
    MpError(String),
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest(String),
    // ========= AUTH =========
    #[error("credenciales inválidas")]
    InvalidCredentials,
    #[error("token inválido")]
    InvalidToken,
    #[error("no autorizado")]
    Unauthorized,
    #[error("permiso denegado")]
    Forbidden,
    // ========= USER =========
    #[error("dni inválido")]
    InvalidDni,
    #[error("nombre inválido")]
    InvalidName,
    #[error("email inválido")]
    InvalidEmail,
    #[error("fecha de nacimiento inválida")]
    InvalidBirthDate,
    #[error("contraseña inválida")]
    InvalidPassword,
    #[error("telefono inválido")]
    InvalidPhone,
    #[error("usuario no encontrado")]
    UserNotFound,
    #[error("contraseña insegura")]
    WeakPassword,
    #[error("email ya registrado")]
    EmailAlreadyExists,
    // ========= DATABASE =========
    #[error("error de base de datos: {0}")]
    DatabaseError(#[from] sqlx::Error),
    // ========= SERVER =========
    #[error("error interno del servidor")]
    InternalServerError,
    #[error("error al iniciar servidor")]
    ServerStartupError,
    // ========= JWT =========
    #[error("error al generar o validar el token JWT: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Error en el token generado")]
    JwtTokenError,
    // ========= HASH =========
    #[error("error procesando contraseña")]
    PasswordHashError,
}
