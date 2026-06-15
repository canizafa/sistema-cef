use super::domain::Empleado;
use crate::app::rol::Rol;
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

#[derive(Debug, Deserialize)]
pub struct UpdatePasswordRequest {
    pub email: String,
    pub password: String,
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

impl From<Empleado> for EmpleadoResponse {
    fn from(empleado: Empleado) -> Self {
        Self {
            dni: empleado.get_dni(),
            nombre_apellido: empleado.get_nombre_apellido(),
            mail: empleado.get_mail(),
            genero: empleado.get_genero(),
            estado: empleado.get_estado(),
            rol: empleado.get_rol(),
        }
    }
}
