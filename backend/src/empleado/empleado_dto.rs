use super::*;
use crate::app::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateEmpleadoRequest {
    pub dni: i64,
    pub nombre_apellido: String,
    pub password: String,
    pub mail: String,
    pub genero: String,
    pub estado: String,
    pub rol: Rol,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEmpleadoRequest {
    pub dni: i64,
    pub nombre_apellido: String,
    pub mail: String,
    pub genero: String,
    pub estado: String,
    pub rol: Rol,
}

#[derive(Debug, Serialize)]
pub struct EmpleadoResponse {
    pub dni: i64,
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

impl From<Empleado> for EmpleadoResponse {
    fn from(empleado: Empleado) -> Self {
        Self {
            dni: empleado.get_dni(),
            nombre_apellido: empleado.get_nombre_apellido(),
            mail: empleado.get_email(),
            genero: empleado.get_genero(),
            estado: empleado.get_estado(),
            rol: empleado.get_rol(),
        }
    }
}
