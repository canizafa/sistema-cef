use thiserror::Error;

use crate::app::errors::FieldError;

#[derive(Debug, Error)]
pub enum ProfesorDomainError {
    #[error("dni must be greater than 0")]
    DniInvalid,
    #[error("nombre_completo cannot be empty")]
    NombreCompletoEmpty,
}

impl From<ProfesorDomainError> for FieldError {
    fn from(error: ProfesorDomainError) -> Self {
        let (field, message) = match error {
            ProfesorDomainError::DniInvalid => ("dni", "dni must be greater than 0".to_string()),
            ProfesorDomainError::NombreCompletoEmpty => (
                "nombre_completo",
                "nombre_completo cannot be empty".to_string(),
            ),
        };
        Self {
            field: field.to_string(),
            message,
        }
    }
}
