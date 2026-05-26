use serde::{Deserialize, Serialize};

use crate::domain::Actividad;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateActividadRequest {
    pub nombre: String,
    pub descripcion: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ActividadResponse {
    pub id: String,
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
