use serde::{Deserialize, Serialize};

use crate::domain::{Clase, rol::Estado};
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct CreateClaseRequest {
    pub dia: NaiveDate,
    pub horario: String,
    pub cupo_profe: i32,
    pub cupo_maximo: i32,
    pub estado: Estado,
    pub id_actividad: i32,
    pub id_sala: i32,
    pub dni_profesor: i32,
    pub descripcion: String,
}

#[derive(Debug, Serialize)]
pub struct ClaseResponse {
    pub id_clase: String,
    pub dia: NaiveDate,
    pub horario: String,
    pub estado: Estado,
    pub lleno: bool,
    pub descripcion: String,
}

#[derive(Debug, Serialize)]
pub struct ClaseListResponse {
    pub clases: Vec<ClaseResponse>,
}

impl From<Clase> for ClaseResponse {
    fn from(value: Clase) -> Self {
        Self {
            id_clase: value.get_id(),
            dia: value.get_dia(),
            horario: value.get_horario(),
            estado: value.get_estado(),
            lleno: value.is_lleno(),
            descripcion: value.get_descripcion(),
        }
    }
}

impl From<Vec<Clase>> for ClaseListResponse {
    fn from(value: Vec<Clase>) -> Self {
        Self {
            clases: value.into_iter().map(|c| c.into()).collect(),
        }
    }
}
