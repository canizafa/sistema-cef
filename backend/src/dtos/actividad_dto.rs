use serde::{Deserialize, Serialize};

use crate::domain::Actividad;

#[derive(Debug, Deserialize)]
pub struct CreateActividadRequest {
    pub nombre: String,
    pub descripcion: String,
}

#[derive(Debug, Serialize)]
pub struct ActividadResponse {
    pub id: i32,
    pub nombre: String,
    pub descripcion: String,
}

impl From<Actividad> for ActividadResponse {
    fn from(value: Actividad) -> Self {
        Self {
            id: value.id.parse().unwrap(),
            nombre: value.nombre,
            descripcion: value.descripcion,
        }
    }
}
