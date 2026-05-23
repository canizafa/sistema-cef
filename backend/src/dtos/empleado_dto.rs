use serde::{Deserialize, Serialize};

use crate::domain::Rol;

#[derive(Debug, Deserialize)]
pub struct CreateEmpleadoRequest {
    pub dni: i32,
    pub nombre_apellido: String,
    pub mail: String,
    pub genero: String,
    pub estado: String,
    pub rol: Rol,
}

#[derive(Debug, Serialize)]
pub struct EmpleadoResponse {
    pub dni: i32,
    pub nombre_apellido: String,
    pub mail: String,
    pub genero: String,
    pub estado: String,
    pub rol: Rol,
}

#[derive(Debug, Serialize)]
pub struct EmpleadoListResponse {
    pub empleados: Vec<EmpleadoResponse>,
}
