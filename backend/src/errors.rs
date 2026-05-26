use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Hubo un error en la base de datos")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Hubo error en la API")]
    Api(ApiError),
    #[error("Hubo un error interno del servidor")]
    InternalServerError,
    #[error("Hubo un error en la migración de la base de datos")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[error("Variable de entorno no encontrada")]
    EnvironmentVariableNotFound,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("asistencia inválida")]
    InvalidAsistencia,

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
    #[error("error generando token")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Error en el token generado")]
    JwtTokenError,

    // ========= HASH =========
    #[error("error procesando contraseña")]
    PasswordHashError,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::InvalidAsistencia => StatusCode::BAD_REQUEST,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,

            ApiError::NotFound => StatusCode::NOT_FOUND,

            ApiError::InvalidDni => StatusCode::BAD_REQUEST,
            ApiError::InvalidName => StatusCode::BAD_REQUEST,
            ApiError::InvalidEmail => StatusCode::BAD_REQUEST,
            ApiError::InvalidBirthDate => StatusCode::BAD_REQUEST,
            ApiError::InvalidPassword => StatusCode::BAD_REQUEST,
            ApiError::InvalidPhone => StatusCode::BAD_REQUEST,
            // ===== AUTH =====
            ApiError::InvalidCredentials => StatusCode::UNAUTHORIZED,

            ApiError::InvalidToken => StatusCode::UNAUTHORIZED,

            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,

            ApiError::Forbidden => StatusCode::FORBIDDEN,

            // ===== USER =====
            ApiError::UserNotFound => StatusCode::NOT_FOUND,

            ApiError::WeakPassword => StatusCode::BAD_REQUEST,

            ApiError::EmailAlreadyExists => StatusCode::CONFLICT,

            // ===== DATABASE =====
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            // ===== SERVER =====
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,

            ApiError::ServerStartupError => StatusCode::INTERNAL_SERVER_ERROR,

            // ===== JWT =====
            ApiError::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            // ===== HASH =====
            ApiError::PasswordHashError => StatusCode::INTERNAL_SERVER_ERROR,

            ApiError::JwtTokenError => StatusCode::UNAUTHORIZED,
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}
