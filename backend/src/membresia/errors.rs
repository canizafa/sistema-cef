use crate::app::errors::FieldError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MembresiaDomainError {
    #[error("Fecha de inicio no puede ser posterior a la fecha de fin")]
    FechaInicioPosteriorAFechaFin,
    #[error("Fecha de inicio no puede ser anterior a 1900-01-01")]
    FechaInicioAnteriorA1900,
    #[error("id_membresia no puede estar vacío")]
    IdMembresiaVacio,
}

impl From<MembresiaDomainError> for FieldError {
    fn from(errors: MembresiaDomainError) -> Self {
        let (field, message) = match errors {
            MembresiaDomainError::FechaInicioPosteriorAFechaFin => (
                "fecha_inicio",
                "Fecha de inicio no puede ser posterior a la fecha de fin",
            ),
            MembresiaDomainError::FechaInicioAnteriorA1900 => (
                "fecha_inicio",
                "Fecha de inicio no puede ser anterior a 1900",
            ),
            MembresiaDomainError::IdMembresiaVacio => {
                ("id_membresia", "id_membresia no puede estar vacío")
            }
        };
        FieldError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}
