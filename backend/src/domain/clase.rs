use chrono::NaiveDate;

use crate::domain::Estado;

pub struct Clase {
    id_clase: String,
    dia: NaiveDate,
    horario: String,
    cupo_profe: i32,
    cupo_maximo: i32,
    estado: Estado,
    descripcion: String,
    id_sala: String,
    dni_profesor: i32,
}

impl Clase {
    pub fn get_id(&self) -> &str {
        &self.id_clase
    }
    pub fn get_dia(&self) -> NaiveDate {
        self.dia
    }
    pub fn get_horario(&self) -> &str {
        &self.horario
    }
    pub fn get_cupo_profe(&self) -> i32 {
        self.cupo_profe
    }
    pub fn get_cupo_maximo(&self) -> i32 {
        self.cupo_maximo
    }
    pub fn get_estado(&self) -> &Estado {
        &self.estado
    }
    pub fn get_descripcion(&self) -> &str {
        &self.descripcion
    }
    pub fn get_id_sala(&self) -> &str {
        &self.id_sala
    }
    pub fn get_dni_profesor(&self) -> i32 {
        self.dni_profesor
    }
}
