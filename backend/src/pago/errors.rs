use thiserror::Error;

use crate::app::errors::FieldError;

#[derive(Debug, Error)]
pub enum PagoDomainError {
    #[error("El monto debe ser mayor a 0")]
    MontoInvalido,
}

impl From<PagoDomainError> for FieldError {
    fn from(error: PagoDomainError) -> Self {
        let (field, message) = match error {
            PagoDomainError::MontoInvalido => (
                "monto".to_string(),
                "El monto debe ser mayor a 0".to_string(),
            ),
        };
        FieldError { field, message }
    }
}
