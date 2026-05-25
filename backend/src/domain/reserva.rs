use chrono::NaiveDate;
use uuid::Uuid;

use crate::{domain::Estado, dtos::CreateReservaRequest, errors::ApiError};

#[derive(Debug, Clone)]
pub struct Reserva {
    id_reserva: String,
    estado: Estado,
    tipo: String,
    fecha_reserva: NaiveDate,
    dni_cliente: String,
    id_clase: String,
}

impl Reserva {
    pub fn get_id(&self) -> &str {
        &self.id_reserva
    }

    pub fn get_estado(&self) -> Estado {
        self.estado.clone()
    }

    pub fn get_tipo(&self) -> String {
        self.tipo.clone()
    }

    pub fn get_fecha_reserva(&self) -> NaiveDate {
        self.fecha_reserva.clone()
    }

    pub fn get_id_clase(&self) -> String {
        self.id_clase.clone()
    }

    pub fn get_dni_cliente(&self) -> String {
        self.dni_cliente.clone()
    }

    pub fn validate_reserva(&self) -> Result<(), ApiError> {
        if self.tipo.is_empty() {
            return Err(ApiError::BadRequest("tipo is required".to_string()));
        }
        if self.fecha_reserva == NaiveDate::MIN {
            return Err(ApiError::BadRequest(
                "fecha_reserva is required".to_string(),
            ));
        }
        if self.dni_cliente.is_empty() {
            return Err(ApiError::BadRequest("dni_cliente is required".to_string()));
        }
        if self.id_clase.is_empty() {
            return Err(ApiError::BadRequest("id_clase is required".to_string()));
        }

        Ok(())
    }
}

impl From<CreateReservaRequest> for Reserva {
    fn from(request: CreateReservaRequest) -> Self {
        Self {
            id_reserva: Uuid::new_v4().to_string(),
            estado: request.estado,
            tipo: request.tipo,
            fecha_reserva: request.fecha,
            dni_cliente: request.dni_cliente,
            id_clase: request.id_clase,
        }
    }
}
