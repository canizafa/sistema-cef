use thiserror::Error;

use crate::app::errors::{AppError, FieldError};

#[derive(Debug, Error)]
pub enum ClienteDomainError {
    #[error("Contraseña muy corta")]
    WeakPassword,
    #[error("DNI inválido")]
    InvalidDni,
    #[error("DNI demasiado corto")]
    DniTooShort,
    #[error("Nombre inválido")]
    InvalidName,
    #[error("Email inválido")]
    InvalidEmail,
    #[error("Teléfono inválido")]
    InvalidPhone,
    #[error("Fecha de nacimiento inválida")]
    InvalidBirthDate,
}

impl From<ClienteDomainError> for FieldError {
    fn from(error: ClienteDomainError) -> Self {
        let (field, message) = match error {
            ClienteDomainError::DniTooShort => ("dni", "DNI demasiado corto".to_string()),
            ClienteDomainError::WeakPassword => ("password", "Contraseña muy corta".to_string()),
            ClienteDomainError::InvalidDni => ("dni", "DNI no puede ser vacío".to_string()),
            ClienteDomainError::InvalidName => ("name", "Nombre no puede ser vacío".to_string()),
            ClienteDomainError::InvalidEmail => ("email", "Email no puede ser vacío".to_string()),
            ClienteDomainError::InvalidPhone => {
                ("phone", "Teléfono no puede ser vacío".to_string())
            }
            ClienteDomainError::InvalidBirthDate => ("birth_date", "Edad insuficiente".to_string()),
        };
        FieldError {
            field: field.to_string(),
            message,
        }
    }
}
impl From<Vec<ClienteDomainError>> for AppError {
    fn from(errors: Vec<ClienteDomainError>) -> Self {
        AppError::Validation(errors.into_iter().map(FieldError::from).collect())
    }
}
