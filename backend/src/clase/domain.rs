use std::str::FromStr;

use crate::{
    clase::estado::EstadoClase,
    clase::{dto::CreateClaseRequest, errors::ClaseDomainError},
};
use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct Clase {
    id_clase: String,
    dia: NaiveDate,
    horario: String,
    descripcion: String,
    cupo_base: i64,
    inscripciones: i64,
    estado: EstadoClase,
    id_sala: String,
    dni_profesor: i64,
    id_actividad: String,
}

impl Clase {
    pub fn new(
        id_clase: String,
        dia: NaiveDate,
        horario: String,
        descripcion: String,
        cupo_base: i64,
        inscripciones: i64,
        estado: EstadoClase,
        id_sala: String,
        dni_profesor: i64,
        id_actividad: String,
    ) -> Self {
        Self {
            id_clase,
            dia,
            horario,
            descripcion,
            cupo_base,
            inscripciones,
            estado,
            id_sala,
            dni_profesor,
            id_actividad,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id_clase
    }
    pub fn get_dia(&self) -> NaiveDate {
        self.dia
    }
    pub fn get_horario(&self) -> &str {
        &self.horario
    }
    pub fn get_cupo_base(&self) -> i64 {
        self.cupo_base
    }
    pub fn get_inscripciones(&self) -> i64 {
        self.inscripciones
    }
    pub fn get_estado(&self) -> &EstadoClase {
        &self.estado
    }
    pub fn get_id_sala(&self) -> &str {
        &self.id_sala
    }
    pub fn get_descripcion(&self) -> &str {
        &self.descripcion
    }
    pub fn get_id_actividad(&self) -> &str {
        &self.id_actividad
    }
    pub fn get_dni_profesor(&self) -> i64 {
        self.dni_profesor
    }
    pub fn verificar_disponibilidad(&self, otras_clases: &[Clase]) -> Vec<ClaseDomainError> {
        let mut errors = Vec::new();
        if otras_clases.iter().any(|c| {
            // si no hay una diferencia de 2 horas entre los horarios
            c.id_sala == self.id_sala
                && NaiveTime::from_str(&c.get_horario())
                    .unwrap()
                    .signed_duration_since(NaiveTime::from_str(&self.horario).unwrap())
                    .abs()
                    < chrono::Duration::hours(2)
                && c.get_dia() == self.dia
        }) {
            errors.push(ClaseDomainError::NoDisponible);
        }
        errors
    }
    pub fn profesor_libre(&self, otras_clases: &[Clase]) -> Vec<ClaseDomainError> {
        let mut errors = Vec::new();
        if otras_clases.iter().any(|c| {
            c.get_dni_profesor() == self.dni_profesor
                && c.get_dia() == self.dia
                && NaiveTime::from_str(&c.get_horario())
                    .unwrap()
                    .signed_duration_since(NaiveTime::from_str(&self.horario).unwrap())
                    .abs()
                    < chrono::Duration::hours(2)
        }) {
            errors.push(ClaseDomainError::ProfesorNoDisponible);
        }
        errors
    }

    pub fn validate_clase(&self, sala_capacidad: i64) -> Vec<ClaseDomainError> {
        let mut errors = Vec::new();
        if self.cupo_base > sala_capacidad {
            errors.push(ClaseDomainError::SalaSobrepasada);
        }
        if self.cupo_base < 0 {
            errors.push(ClaseDomainError::CupoNegativo);
        }
        if self.horario.trim().is_empty() {
            errors.push(ClaseDomainError::HorarioInvalido);
        }
        if self.descripcion.trim().is_empty() {
            errors.push(ClaseDomainError::DescripcionInvalida);
        }
        if self.id_sala.trim().is_empty() {
            errors.push(ClaseDomainError::IdSalaInvalido);
        }
        if self.dni_profesor <= 0 {
            errors.push(ClaseDomainError::DniProfesorInvalido);
        }
        if self.id_actividad.trim().is_empty() {
            errors.push(ClaseDomainError::IdActividadInvalido);
        }
        errors
    }

    pub fn update_clase(&mut self, estado: EstadoClase, dni_profesor: i64) {
        self.estado = estado;
        self.dni_profesor = dni_profesor;
    }

    pub fn is_lleno(&self) -> bool {
        self.inscripciones >= self.cupo_base
    }

    pub fn aumentar_inscripciones(&mut self, sala_capacidad: i64) {
        let _ = match self.estado {
            EstadoClase::SinCupo => {}
            EstadoClase::Extendido => {
                if self.cupo_base + self.inscripciones < sala_capacidad {
                    self.inscripciones += 1;
                }
            }
            EstadoClase::Alta => {
                if self.cupo_base > 0 {
                    self.inscripciones += 1;
                }
            }
        };
    }

    pub fn decrementar_inscripciones(&mut self) {
        if self.inscripciones > 0 {
            self.inscripciones -= 1;
        }
    }

    pub fn extender_cupo(&mut self) {
        self.estado = EstadoClase::Extendido;
    }
}

impl From<CreateClaseRequest> for Clase {
    fn from(value: CreateClaseRequest) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            id_clase: id,
            dia: value.dia,
            horario: value.horario,
            descripcion: value.descripcion,
            inscripciones: 0,
            cupo_base: value.cupo_base,
            estado: value.estado,
            id_sala: value.id_sala,
            dni_profesor: value.dni_profesor,
            id_actividad: value.id_actividad,
        }
    }
}
