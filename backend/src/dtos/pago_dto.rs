use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::dtos::ReservaResponse;

#[derive(Debug, Deserialize)]
pub struct CreatePagoRequest {
    pub monto: f64,
    pub fecha: NaiveDate,
    pub hora: String,
    pub sena: bool,
    pub id_membresia: i32,
}

#[derive(Debug, Serialize)]
pub struct PagoResponse {
    pub id_pago: i32,
    pub monto: f64,
    pub fecha: NaiveDate,
    pub hora: String,
    pub sena: bool,
    pub id_membresia: i32,
    pub reserva_paga: ReservaResponse,
}

#[derive(Debug, Serialize)]
pub struct PagoListResponse {
    pub pagos: Vec<PagoResponse>,
}
