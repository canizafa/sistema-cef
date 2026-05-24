use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::rol::{Estado, Rol};
use crate::dtos::{CreateFichaMedicaRequest, FichaMedicaResponse};

#[derive(Debug, Deserialize)]
pub struct CreateClienteRequest {
    pub dni: i32,
    pub nombre_apellido: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
    pub estado: Estado,
    pub ficha_medica: CreateFichaMedicaRequest,
}

#[derive(Debug, Serialize)]
pub struct ClienteResponse {
    pub dni: i32,
    pub nombre_apellido: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
    pub estado: Estado,
    pub rol: Rol,
    pub ficha_medica: FichaMedicaResponse,
}

#[derive(Debug, Serialize)]
pub struct ClienteListResponse {
    pub clientes: Vec<ClienteResponse>,
}
