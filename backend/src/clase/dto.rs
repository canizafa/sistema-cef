use super::domain::Clase;
use crate::clase::estado::EstadoClase;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateClaseRequest {
    pub dia: NaiveDate,
    pub horario: String,
    pub cupo_base: i64,
    pub estado: EstadoClase,
    pub id_actividad: String,
    pub id_sala: String,
    pub dni_profesor: i64,
    pub descripcion: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClaseRequest {
    pub id_clase: String,
    pub dni_profesor: i64,
    pub estado: EstadoClase,
}

#[derive(Debug, Serialize)]
pub struct ClaseResponse {
    pub id_clase: String,
    pub dia: NaiveDate,
    pub dia_semana: String,
    pub horario: String,
    pub cupo_base: i64,
    pub inscripciones: i64,
    pub estado: EstadoClase,
    pub lleno: bool,
    pub dni_profesor: i64,
    pub descripcion: String,
    pub id_actividad: String,
    pub id_sala: String,
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
            inscripciones: value.get_inscripciones(),
            estado: value.get_estado().to_owned(),
            lleno: value.is_lleno(),
            dni_profesor: value.get_dni_profesor(),
            descripcion: value.get_descripcion().to_owned(),
            id_actividad: value.get_id_actividad().to_owned(),
            id_sala: value.get_id_sala().to_string(),
        }
    }
}

impl From<(UpdateClaseRequest, Clase)> for Clase {
    fn from(value: (UpdateClaseRequest, Clase)) -> Self {
        Clase::new(
            value.0.id_clase,
            value.1.get_dia(),
            value.1.get_horario().to_owned(),
            value.1.get_descripcion().to_owned(),
            value.1.get_cupo_base(),
            value.1.get_inscripciones(),
            value.0.estado,
            value.1.get_id_sala().to_string(),
            value.0.dni_profesor,
            value.1.get_id_actividad().to_owned(),
        )
    }
}
