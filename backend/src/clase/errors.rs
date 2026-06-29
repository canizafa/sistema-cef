use thiserror::Error;

use crate::{app::errors::FieldError, clase::errors::ClaseDomainError::ProfesorNoDisponible};

#[derive(Debug, Error)]
pub enum ClaseDomainError {
    #[error("Se supera el límite de la sala")]
    SalaSobrepasada,
    #[error("Cupo negativo")]
    CupoNegativo,
    #[error("Horario inválido")]
    HorarioInvalido,
    #[error("Día invalido")]
    DiaInvalido,
    #[error("Descripción inválida")]
    DescripcionInvalida,
    #[error("ID de sala inválido")]
    IdSalaInvalido,
    #[error("DNI de profesor inválido")]
    DniProfesorInvalido,
    #[error("ID de actividad inválido")]
    IdActividadInvalido,
    #[error("No disponible")]
    NoDisponible,
    #[error("Profesor no disponible")]
    ProfesorNoDisponible,
}

impl From<ClaseDomainError> for FieldError {
    fn from(error: ClaseDomainError) -> Self {
        let (field, message) = match error {
            ClaseDomainError::DiaInvalido => {
                ("dia", "No se puede crear la clase en ese día".to_string())
            }
            ProfesorNoDisponible => (
                "profesor",
                "Profesor no disponible en ese día y horario".to_string(),
            ),
            ClaseDomainError::SalaSobrepasada => {
                ("sala", "Se supera el límite de la sala".to_string())
            }
            ClaseDomainError::CupoNegativo => ("cupo", "Cupo negativo".to_string()),
            ClaseDomainError::HorarioInvalido => ("horario", "Horario inválido".to_string()),
            ClaseDomainError::DescripcionInvalida => {
                ("descripcion", "Descripción inválida".to_string())
            }
            ClaseDomainError::IdSalaInvalido => ("id_sala", "ID de sala inválido".to_string()),
            ClaseDomainError::DniProfesorInvalido => {
                ("dni_profesor", "DNI de profesor inválido".to_string())
            }
            ClaseDomainError::IdActividadInvalido => {
                ("id_actividad", "ID de actividad inválido".to_string())
            }
            ClaseDomainError::NoDisponible => (
                "no_disponible",
                "Sala no disponible para ese día y horario".to_string(),
            ),
        };
        FieldError {
            field: field.to_string(),
            message,
        }
    }
}
