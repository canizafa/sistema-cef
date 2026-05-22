use serde::{Deserialize, Serialize};

use crate::domain::rol::Estado;

#[derive(Debug, Deserialize)]
pub struct CreateClaseRequest {
    pub dia: String,
    pub horario: String,
    pub cupo_profe: i32,
    pub cupo_maximo: i32,
    pub estado: Estado,
    pub id_actividad: i32,
    pub id_sala: i32,
    pub dni_profesor: i32,
}

#[derive(Debug, Serialize)]
pub struct ClaseResponse {
    pub id_clase: i32,
    pub dia: String,
    pub horario: String,
    pub estado: Estado,
}
