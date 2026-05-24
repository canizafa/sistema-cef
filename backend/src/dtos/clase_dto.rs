use serde::{Deserialize, Serialize};

use crate::domain::rol::Estado;
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
