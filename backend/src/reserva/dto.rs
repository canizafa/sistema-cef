use crate::reserva::{domain::Reserva, estado::EstadoReserva};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct CreateReservaRequest {
    pub fecha: NaiveDate,
    pub tipo: String,
    pub estado: EstadoReserva,
    pub dni_cliente: i64,
    pub id_clase: String,
    pub horario: String,
}

#[derive(Debug, Serialize)]
pub struct ReservaResponse {
    pub id_reserva: String,
    pub fecha: NaiveDate,
    pub tipo: String,
    pub estado: EstadoReserva,
    pub dni_cliente: i64,
    pub id_clase: String,
    pub horario: String,
}

#[derive(Debug, Serialize)]
pub struct ReservaListResponse {
    pub reservas: Vec<ReservaResponse>,
}

impl From<Reserva> for ReservaResponse {
    fn from(reserva: Reserva) -> Self {
        Self {
            id_reserva: reserva.get_id().to_string(),
            fecha: reserva.get_fecha_reserva(),
            tipo: reserva.get_tipo(),
            estado: reserva.get_estado(),
            dni_cliente: reserva.get_dni_cliente(),
            id_clase: reserva.get_id_clase(),
            horario: reserva.get_horario().to_owned(),
        }
    }
}

impl From<Reserva> for CreateReservaRequest {
    fn from(reserva: Reserva) -> Self {
        Self {
            fecha: reserva.get_fecha_reserva(),
            tipo: reserva.get_tipo(),
            estado: reserva.get_estado(),
            dni_cliente: reserva.get_dni_cliente(),
            id_clase: reserva.get_id_clase(),
            horario: reserva.get_horario().to_owned(),
        }
    }
}
