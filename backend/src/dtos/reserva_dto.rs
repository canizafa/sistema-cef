use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::rol::Estado;

#[derive(Debug, Deserialize)]
pub struct CreateReservaRequest {
    pub fecha: NaiveDate,
    pub estado: Estado,
    pub dni_cliente: i32,
    pub id_clase: i32,
}

#[derive(Debug, Serialize)]
pub struct ReservaResponse {
    pub fecha: NaiveDate,
    pub estado: Estado,
    pub dni_cliente: i32,
    pub id_clase: i32,
}

#[derive(Debug, Serialize)]
pub struct ReservaListResponse {
    pub reservas: Vec<ReservaResponse>,
}
