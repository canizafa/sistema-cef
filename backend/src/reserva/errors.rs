use crate::app::errors::FieldError;

#[derive(Debug, thiserror::Error)]
pub enum ReservaDomainError {
    #[error("tipo is required")]
    Tipo,
    #[error("fecha_reserva is required")]
    FechaReserva,
    #[error("dni_cliente is required")]
    DniCliente,
    #[error("id_clase is required")]
    IdClase,
}

impl From<ReservaDomainError> for FieldError {
    fn from(error: ReservaDomainError) -> Self {
        let (field, message) = match error {
            ReservaDomainError::DniCliente => (
                "dni_cliente".to_string(),
                "dni_cliente is required".to_string(),
            ),
            ReservaDomainError::Tipo => ("tipo".to_string(), "tipo es requerido".to_string()),
            ReservaDomainError::FechaReserva => (
                "fecha_reserva".to_string(),
                "fecha_reserva es requerido".to_string(),
            ),
            ReservaDomainError::IdClase => {
                ("id_clase".to_string(), "id_clase es requerido".to_string())
            }
        };
        FieldError { field, message }
    }
}
