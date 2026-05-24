use chrono::NaiveDate;

use crate::{domain::Reserva, dtos::CreatePagoRequest};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Pago {
    id_pago: String,
    monto: f64,
    fecha: NaiveDate,
    hora: String,
    sena: bool,
    id_membresia: Option<String>,
    reserva_paga: Option<String>,
}

impl Pago {
    pub fn get_id_pago(&self) -> &str {
        &self.id_pago
    }

    pub fn get_monto(&self) -> f64 {
        self.monto
    }

    pub fn get_fecha(&self) -> NaiveDate {
        self.fecha
    }

    pub fn get_hora(&self) -> String {
        self.hora.clone()
    }

    pub fn get_sena(&self) -> bool {
        self.sena
    }

    pub fn get_id_membresia(&self) -> Option<&String> {
        self.id_membresia.as_ref()
    }

    pub fn get_reserva_paga(&self) -> Option<&String> {
        self.reserva_paga.as_ref()
    }
}

impl From<CreatePagoRequest> for Pago {
    fn from(request: CreatePagoRequest) -> Self {
        Self {
            id_pago: Uuid::new_v4().to_string(),
            monto: request.monto,
            fecha: request.fecha,
            hora: request.hora,
            sena: request.sena,
            id_membresia: request.id_membresia.to_string(),
            reserva_paga: request.reserva_paga.into(),
        }
    }
}
