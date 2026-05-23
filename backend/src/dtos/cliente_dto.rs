use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::rol::Estado;

#[derive(Debug, Deserialize)]
pub struct CreateClienteRequest {
    pub dni: i32,
    pub nombre_apellido: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
    pub estado: Estado,
    // pub ficha_medica:
}

#[derive(Debug, Serialize)]
pub struct ClienteResponse {
    pub dni: i32,
    pub nombre_apellido: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
    pub estado: Estado,
}

#[derive(Debug, Serialize)]
pub struct ClienteListResponse {
    pub clientes: Vec<ClienteResponse>,
}
