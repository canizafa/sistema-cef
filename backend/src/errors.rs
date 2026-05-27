use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
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

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            AppError::Api(api_error) => {
                return api_error.into_response();
            }

            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            AppError::MigrationError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            AppError::EnvironmentVariableNotFound => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
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

impl From<AppError> for Response {
    fn from(error: AppError) -> Self {
        let status = match error {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Api(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::MigrationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EnvironmentVariableNotFound => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Json(ErrorResponse {
            error: error.to_string(),
        })
        .into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::SmtpError(_) => StatusCode::BAD_GATEWAY,
            ApiError::AddressError(_) => StatusCode::BAD_REQUEST,
            ApiError::MessageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
