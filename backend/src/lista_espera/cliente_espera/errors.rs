use thiserror::Error;

use crate::app::errors::FieldError;

#[derive(Debug, Error)]
pub enum ClienteListaEsperaDomainError {
    #[error("DNI invalido")]
    ClienteNotFound,
    #[error("Lista de espera invalida")]
    IdEsperaEmpty,
}

impl From<ClienteListaEsperaDomainError> for FieldError {
    fn from(error: ClienteListaEsperaDomainError) -> Self {
        let (field, message) = match error {
            ClienteListaEsperaDomainError::ClienteNotFound => ("dni", "DNI invalido"),
            ClienteListaEsperaDomainError::IdEsperaEmpty => {
                ("id_espera", "Lista de espera invalida")
            }
        };
        FieldError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}
