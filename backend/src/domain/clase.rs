use chrono::NaiveDate;
use uuid::Uuid;

use crate::{domain::Estado, dtos::CreateClaseRequest};

#[derive(Debug, Clone)]
pub struct Clase {
    id_clase: String,
    dia: NaiveDate,
    horario: String,
    cupo_profe: i32,
    cupo_maximo: i32,
    estado: Estado,
    id_sala: String,
    dni_profesor: String,
}

impl Clase {
    pub fn get_id(&self) -> &str {
        &self.id_clase
    }
    pub fn get_dia(&self) -> NaiveDate {
        self.dia
    }
    pub fn get_horario(&self) -> String {
        self.horario.clone()
    }
    pub fn get_cupo_profe(&self) -> i32 {
        self.cupo_profe
    }
    pub fn get_cupo_maximo(&self) -> i32 {
        self.cupo_maximo
    }
    pub fn get_estado(&self) -> Estado {
        self.estado.clone()
    }

    pub fn is_lleno(&self) -> bool {
        self.cupo_profe >= self.cupo_maximo
    }

    pub fn get_id_sala(&self) -> &str {
        &self.id_sala
    }
}

impl From<CreateClaseRequest> for Clase {
    fn from(value: CreateClaseRequest) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            id_clase: id,
            dia: value.dia,
            horario: value.horario,
            cupo_profe: value.cupo_profe,
            cupo_maximo: value.cupo_maximo,
            estado: value.estado,
            id_sala: value.id_sala,
            dni_profesor: value.dni_profesor,
        }
    }
}
