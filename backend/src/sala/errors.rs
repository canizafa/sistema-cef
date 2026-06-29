use thiserror::Error;

use crate::app::errors::FieldError;

#[derive(Debug, Error)]
pub enum SalaDomainError {
    #[error("Numero invalido")]
    NumeroInvalido,
    #[error("Capacidad invalida")]
    CapacidadInvalida,
}

impl From<SalaDomainError> for FieldError {
    fn from(error: SalaDomainError) -> Self {
        let (field, message) = match error {
            SalaDomainError::NumeroInvalido => ("numero", "Numero invalido"),
            SalaDomainError::CapacidadInvalida => ("capacidad", "Capacidad invalida"),
        };
        FieldError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}
