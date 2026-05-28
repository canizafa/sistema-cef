use std::str::FromStr;

use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

use crate::{domain::Estado, dtos::CreateClaseRequest, errors::ApiError};

#[derive(Debug, Clone)]
pub struct Clase {
    id_clase: String,
    dia: NaiveDate,
    horario: String,
    descripcion: String,
    cupo_maximo: i64,
    cupo_base: i64,
    estado: Estado,
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
        cupo_maximo: i64,
        estado: Estado,
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
            cupo_maximo,
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
    pub fn get_horario(&self) -> String {
        self.horario.clone()
    }
    pub fn get_cupo_base(&self) -> i64 {
        self.cupo_base
    }
    pub fn get_cupo_maximo(&self) -> i64 {
        self.cupo_maximo
    }
    pub fn get_estado(&self) -> Estado {
        self.estado.clone()
    }

    pub fn get_id_sala(&self) -> &str {
        &self.id_sala
    }
    pub fn get_descripcion(&self) -> String {
        self.descripcion.clone()
    }
    pub fn get_id_actividad(&self) -> String {
        self.id_actividad.clone()
    }
    pub fn get_dni_profesor(&self) -> i64 {
        self.dni_profesor
    }

    pub fn sala_libre(&self, otras_clases: &[Clase]) -> Result<(), ApiError> {
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
            Err(ApiError::BadRequest("La sala ya está tomada".to_string()))
        } else {
            Ok(())
        }
    }
    pub fn profesor_libre(&self, otras_clases: &[Clase]) -> Result<(), ApiError> {
        if otras_clases.iter().any(|c| {
            c.get_dni_profesor() == self.dni_profesor
                && c.get_dia() == self.dia
                && NaiveTime::from_str(&c.get_horario())
                    .unwrap()
                    .signed_duration_since(NaiveTime::from_str(&self.horario).unwrap())
                    .abs()
                    < chrono::Duration::hours(2)
        }) {
            Err(ApiError::BadRequest(
                "El profesor ya está tomado".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn validate_clase(&self) -> Result<(), ApiError> {
        if self.cupo_base > self.cupo_maximo {
            return Err(ApiError::BadRequest(
                "El cupo base no puede exceder el cupo máximo".to_string(),
            ));
        }

        if self.cupo_base < 0 {
            return Err(ApiError::BadRequest(
                "El cupo base no puede ser negativo".to_string(),
            ));
        }

        if self.cupo_maximo < 0 {
            return Err(ApiError::BadRequest(
                "El cupo máximo no puede ser negativo".to_string(),
            ));
        }

        if self.horario.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "El horario no puede estar vacío".to_string(),
            ));
        }

        if self.descripcion.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "La descripción no puede estar vacía".to_string(),
            ));
        }

        if self.id_sala.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "La sala no puede estar vacía".to_string(),
            ));
        }

        if self.dni_profesor <= 0 {
            return Err(ApiError::BadRequest(
                "El DNI del profesor debe ser mayor a cero".to_string(),
            ));
        }

        if self.id_actividad.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "La actividad no puede estar vacía".to_string(),
            ));
        }

        Ok(())
    }

    pub fn disminuir_cupo(&mut self) -> Result<(), ApiError> {
        if self.cupo_base <= 0 {
            self.estado = Estado::SinCupo;

            return Err(ApiError::BadRequest("No hay cupo disponible".to_string()));
        }

        self.cupo_base -= 1;

        if self.cupo_base == 0 {
            self.estado = Estado::SinCupo;
        }

        Ok(())
    }

    pub fn is_lleno(&self) -> bool {
        self.cupo_base <= 0 || matches!(self.estado, Estado::SinCupo)
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
            cupo_base: value.cupo_base,
            cupo_maximo: value.cupo_maximo,
            estado: value.estado,
            id_sala: value.id_sala,
            dni_profesor: value.dni_profesor,
            id_actividad: value.id_actividad,
        }
    }
}
