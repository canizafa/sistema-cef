use thiserror::Error;

use crate::app::FieldError;

#[derive(Debug, Error)]
pub enum ActividadDomainError {
    #[error("El nombre de la actividad no puede estar vacío")]
    NombreVacio,
    #[error("La descripción de la actividad no puede estar vacía")]
    DescripcionVacia,
}

impl From<ActividadDomainError> for FieldError {
    fn from(error: ActividadDomainError) -> Self {
        let (field, message) = match error {
            ActividadDomainError::NombreVacio => {
                ("nombre", "El nombre de la actividad no puede estar vacío")
            }
            ActividadDomainError::DescripcionVacia => (
                "descripcion",
                "La descripción de la actividad no puede estar vacía",
            ),
        };
        Self {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}
