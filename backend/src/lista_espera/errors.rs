use thiserror::Error;

use crate::app::errors::FieldError;

#[derive(Debug, Error)]
pub enum ListaEsperaDomainError {
    #[error("tipo invalido")]
    TipoInvalido,
    #[error("id clase invalido")]
    IdClaseInvalido,
}

impl From<ListaEsperaDomainError> for FieldError {
    fn from(error: ListaEsperaDomainError) -> Self {
        let (field, message) = match error {
            ListaEsperaDomainError::TipoInvalido => ("tipo", "tipo invalido"),
            ListaEsperaDomainError::IdClaseInvalido => ("id_clase", "id clase invalido"),
        };
        FieldError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}
