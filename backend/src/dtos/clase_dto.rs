use serde::{Deserialize, Serialize};

use crate::domain::{Clase, rol::Estado};
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct CreateClaseRequest {
    pub dia: NaiveDate,
    pub dia_semana: String,
    pub horario: String,
    pub cupo_maximo: i64,
    pub cupo_base: i64,
    pub estado: Estado,
    pub id_actividad: String,
    pub id_sala: String,
    pub dni_profesor: i64,
    pub descripcion: String,
}

#[derive(Debug, Serialize)]
pub struct ClaseResponse {
    pub id_clase: String,
    pub dia: NaiveDate,
    pub dia_semana: String,
    pub horario: String,
    pub estado: Estado,
    pub lleno: bool,
    pub descripcion: String,
    pub id_actividad: String,
    pub id_sala: String,
}

#[derive(Debug, Serialize)]
pub struct ClaseListResponse {
    pub clases: Vec<ClaseResponse>,
}

impl From<Clase> for ClaseResponse {
    fn from(value: Clase) -> Self {
        Self {
            id_clase: value.get_id().to_string(),
            dia: value.get_dia(),
            dia_semana: value.get_dia().format("%A").to_string(),
            horario: value.get_horario(),
            estado: value.get_estado(),
            lleno: value.is_lleno(),
            descripcion: value.get_descripcion(),
            id_actividad: value.get_id_actividad(),
            id_sala: value.get_id_sala().to_string(),
        }
    }
}
