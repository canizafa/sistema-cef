use chrono::NaiveDate;
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
            c.id_sala == self.id_sala && c.get_horario() == self.horario && c.get_dia() == self.dia
        }) {
            Err(ApiError::BadRequest("La sala ya está tomada".to_string()))
        } else {
            Ok(())
        }
    }

    pub fn validate_clase(&self) -> Result<(), ApiError> {
        if self.cupo_base > self.cupo_maximo {
            return Err(ApiError::BadRequest(
                "El cupo del profesor excede el cupo máximo".to_string(),
            ));
        }
        if self.cupo_base < 0 {
            return Err(ApiError::BadRequest(
                "El cupo del profesor no puede ser negativo".to_string(),
            ));
        }
        if self.cupo_maximo < 0 {
            return Err(ApiError::BadRequest(
                "El cupo máximo no puede ser negativo".to_string(),
            ));
        }
        if self.horario.is_empty() {
            return Err(ApiError::BadRequest(
                "El horario no puede estar vacío".to_string(),
            ));
        }
        if self.descripcion.is_empty() {
            return Err(ApiError::BadRequest(
                "La descripción no puede estar vacía".to_string(),
            ));
        }

        if self.id_sala.is_empty() {
            return Err(ApiError::BadRequest(
                "La sala no puede estar vacía".to_string(),
            ));
        }
        if self.dni_profesor <= 0 {
            return Err(ApiError::BadRequest(
                "El DNI del profesor no puede ser negativo o cero".to_string(),
            ));
        }
        if self.id_actividad.is_empty() {
            return Err(ApiError::BadRequest(
                "La actividad no puede estar vacía".to_string(),
            ));
        }

        Ok(())
    }

    pub fn disminuir_cupo(&mut self) -> Result<(), ApiError> {
        if self.cupo_base > 0 {
            self.cupo_base -= 1;
        } else {
            self.estado = Estado::SinCupo;
            return Err(ApiError::BadRequest("No hay cupo disponible".to_string()));
        }
        Ok(())
    }

    pub fn is_lleno(&self) -> bool {
        match self.estado {
            Estado::Alta | Estado::SinCupo => self.cupo_base >= self.cupo_maximo,
            Estado::Baja => false,
        }
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
