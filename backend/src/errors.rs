use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
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
    #[error("usuario no encontrado")]
    UserNotFound,

    #[error("email inválido")]
    InvalidEmail,

    #[error("contraseña insegura")]
    WeakPassword,

    #[error("email ya registrado")]
    EmailAlreadyExists,

    // ========= DATABASE =========
    #[error("error de base de datos")]
    DatabaseError(#[from] sqlx::Error),

    // ========= SERVER =========
    #[error("error interno del servidor")]
    InternalServerError,

    #[error("error al iniciar servidor")]
    ServerStartupError,

    // ========= JWT =========
    #[error("error generando token")]
    JwtError(#[from] jsonwebtoken::errors::Error),

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
            // ===== AUTH =====
            ApiError::InvalidCredentials => StatusCode::UNAUTHORIZED,

            ApiError::InvalidToken => StatusCode::UNAUTHORIZED,

            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,

            ApiError::Forbidden => StatusCode::FORBIDDEN,

            // ===== USER =====
            ApiError::UserNotFound => StatusCode::NOT_FOUND,

            ApiError::InvalidEmail => StatusCode::BAD_REQUEST,

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
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}
