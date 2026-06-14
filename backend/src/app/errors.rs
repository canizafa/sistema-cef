use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

// --------- ERRORES DE LA BASE DE DATOS --------
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Registro no encontrado")]
    NotFound,
    #[error("Violación de unicidad en : {0}")]
    UniqueViolation(String),
    #[error("Error en la conexión con la base de datos")]
    ConnectionError,
    #[error("Error de query: {0}")]
    QueryError(sqlx::Error),
}

impl From<sqlx::Error> for DbError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => DbError::NotFound,
            sqlx::Error::Database(db_err) => {
                if db_err.code().as_deref() == Some("2067") {
                    DbError::UniqueViolation(db_err.message().into())
                } else {
                    DbError::QueryError(sqlx::Error::Database(db_err))
                }
            }
            sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => DbError::ConnectionError,
            other => DbError::QueryError(other),
        }
    }
}
// --------- ERRORES DE LA APP --------
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error de validacion")]
    Validation(Vec<FieldError>),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("Error interno del servidor")]
    Internal,
    #[error("Servicio no disponible")]
    ServiceUnavailable,
    #[error("Variable de entorno no encontrada")]
    EnvironmentVariableNotFound,
    #[error("Error de hash de contraseña")]
    PasswordHashError,
    #[error("Credenciales invalidas")]
    InvalidCredentials,
    #[error("Token invalido")]
    JwtError,
}
impl From<DbError> for AppError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::NotFound => AppError::NotFound("El recurso solicitado no existe".into()),
            DbError::UniqueViolation(_) => {
                AppError::Conflict("Ya existe un registro con esos datos".into())
            }
            DbError::ConnectionError => AppError::ServiceUnavailable,
            DbError::QueryError(e) => {
                // Loguear acá el error real sin exponerlo al front
                tracing::error!("Error de query inesperado: {:?}", e);
                AppError::Internal
            }
        }
    }
}
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::from(DbError::from(err))
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            AppError::JwtError => (
                StatusCode::UNAUTHORIZED,
                json!({ "error": "Token invalido" }),
            ),
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                json!({ "error": "Credenciales invalidas" }),
            ),
            AppError::PasswordHashError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": "Error de hash de contraseña" }),
            ),
            AppError::EnvironmentVariableNotFound => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": "Variable de entorno no encontrada" }),
            ),
            AppError::Validation(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                json!({
                    "error": "errores de validación",
                    "details": errors
                }),
            ),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, json!({ "error": msg })),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, json!({ "error": msg })),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, json!({ "error": msg })),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, json!({ "error": msg })),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": "error interno del servidor" }),
            ),
            AppError::ServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                json!({ "error": "servicio no disponible, intentá más tarde" }),
            ),
        };

        (status, Json(body)).into_response()
    }
}
