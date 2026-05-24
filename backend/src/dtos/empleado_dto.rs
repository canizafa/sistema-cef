use serde::{Deserialize, Serialize};

use crate::domain::{Empleado, Rol};

#[derive(Debug, Deserialize)]
pub struct CreateEmpleadoRequest {
    pub dni: String,
    pub nombre_apellido: String,
    pub password: String,
    pub mail: String,
    pub genero: String,
    pub estado: String,
    pub rol: Rol,
}

#[derive(Debug, Serialize)]
pub struct EmpleadoResponse {
    pub dni: String,
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
            mail: empleado.get_mail(),
            genero: empleado.get_genero(),
            estado: empleado.get_estado(),
            rol: empleado.get_rol(),
        }
    }
}
