use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::dtos::CreateReservaRequest;

#[derive(Debug, Deserialize, Clone)]
pub struct CreatePagoRequest {
    pub titulo: String,
    pub monto: f64,
    pub fecha: NaiveDate,
    pub hora: String,
    pub sena: bool,
    pub id_membresia: String,
    pub reserva_paga: CreateReservaRequest,
}

#[derive(Debug, Serialize)]
pub struct PagoResponse {
    pub init_point: String,
    pub sandbox_init_point: String,
}
