use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::pago::Pago;
use crate::dtos::CreateReservaRequest;
use crate::dtos::ReservaResponse;

#[derive(Debug, Deserialize)]
pub struct CreatePagoRequest {
    pub monto: f64,
    pub fecha: NaiveDate,
    pub hora: String,
    pub sena: bool,
    pub id_membresia: String,
    pub reserva_paga: CreateReservaRequest,
}

#[derive(Debug, Serialize)]
pub struct PagoResponse {
    pub id_pago: String,
    pub monto: f64,
    pub fecha: NaiveDate,
    pub hora: String,
    pub sena: bool,
    pub id_membresia: String,
    pub reserva_paga: ReservaResponse,
}

#[derive(Debug, Serialize)]
pub struct PagoListResponse {
    pub pagos: Vec<PagoResponse>,
}

impl From<Pago> for PagoResponse {
    fn from(pago: Pago) -> Self {
        Self {
            id_pago: pago.get_id_pago().to_string(),
            monto: pago.get_monto(),
            fecha: pago.get_fecha(),
            hora: pago.get_hora(),
            sena: pago.get_sena(),
            id_membresia: pago.get_id_membresia().to_string(),
            reserva_paga: pago.get_reserva_paga().into(),
        }
    }
}

impl From<Vec<Pago>> for PagoListResponse {
    fn from(pagos: Vec<Pago>) -> Self {
        Self {
            pagos: pagos.into_iter().map(|pago| pago.into()).collect(),
        }
    }
}
