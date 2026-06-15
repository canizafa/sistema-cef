use super::domain::Clase;
use crate::app::rol::Estado;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateClaseRequest {
    pub dia: NaiveDate,
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
    pub cupo_base: i64,
    pub estado: Estado,
    pub lleno: bool,
    pub dni_profesor: i64,
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
        let dia_semana_castellano = match value.get_dia().format("%A").to_string().as_str() {
            "Monday" => "Lunes",
            "Tuesday" => "Martes",
            "Wednesday" => "Miércoles",
            "Thursday" => "Jueves",
            "Friday" => "Viernes",
            "Saturday" => "Sábado",
            "Sunday" => "Domingo",
            _ => panic!(
                "Invalid day of the week: {}",
                value.get_dia().format("%A").to_string()
            ),
        };
        Self {
            id_clase: value.get_id().to_string(),
            dia: value.get_dia(),
            dia_semana: dia_semana_castellano.to_string(),
            horario: value.get_horario().to_owned(),
            cupo_base: value.get_cupo_base(),
            estado: value.get_estado().to_owned(),
            lleno: value.is_lleno(),
            dni_profesor: value.get_dni_profesor(),
            descripcion: value.get_descripcion().to_owned(),
            id_actividad: value.get_id_actividad().to_owned(),
            id_sala: value.get_id_sala().to_string(),
        }
    }
}
