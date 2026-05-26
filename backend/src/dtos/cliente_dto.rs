use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::Cliente;
use crate::domain::rol::{Estado, Rol};
use crate::dtos::{CreateFichaMedicaRequest, FichaMedicaResponse};

#[derive(Debug, Deserialize)]
pub struct CreateClienteRequest {
    pub dni: i64,
    pub nombre_apellido: String,
    pub password: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
    pub estado: Estado,
    pub ficha_medica: CreateFichaMedicaRequest,
}

#[derive(Debug, Serialize)]
pub struct ClienteResponse {
    pub dni: i64,
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

impl From<Cliente> for ClienteResponse {
    fn from(cliente: Cliente) -> Self {
        Self {
            dni: cliente.get_dni(),
            nombre_apellido: cliente.get_nombre_apellido(),
            email: cliente.get_email(),
            telefono: cliente.get_telefono(),
            fecha_nacimiento: cliente.get_fecha_nacimiento(),
            estado: cliente.get_estado(),
            rol: cliente.get_rol(),
            ficha_medica: cliente.get_ficha_medica().into(),
        }
    }
}
