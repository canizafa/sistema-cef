use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::{Reserva, rol::Estado};

#[derive(Debug, Deserialize, Clone)]
pub struct CreateReservaRequest {
    pub fecha: NaiveDate,
    pub tipo: String,
    pub estado: Estado,
    pub dni_cliente: String,
    pub id_clase: String,
}

#[derive(Debug, Serialize)]
pub struct ReservaResponse {
    pub fecha: NaiveDate,
    pub tipo: String,
    pub estado: Estado,
    pub dni_cliente: String,
    pub id_clase: String,
}

#[derive(Debug, Serialize)]
pub struct ReservaListResponse {
    pub reservas: Vec<ReservaResponse>,
}

impl From<Reserva> for ReservaResponse {
    fn from(reserva: Reserva) -> Self {
        Self {
            fecha: reserva.get_fecha_reserva(),
            tipo: reserva.get_tipo(),
            estado: reserva.get_estado(),
            dni_cliente: reserva.get_dni_cliente(),
            id_clase: reserva.get_id_clase(),
        }
    }
}
