use chrono::NaiveDate;
use uuid::Uuid;

use crate::{domain::Estado, dtos::CreateMembresiaRequest, errors::ApiError};

#[derive(Debug, Clone)]
pub struct Membresia {
    id_membresia: String,
    tipo: String,
    estado: Estado,
    fecha_inicio: NaiveDate,
    fecha_fin: Option<NaiveDate>,
}

impl Membresia {
    pub fn new(
        id_membresia: String,
        tipo: String,
        estado: Estado,
        fecha_inicio: NaiveDate,
        fecha_fin: Option<NaiveDate>,
    ) -> Self {
        Self {
            id_membresia,
            tipo,
            estado,
            fecha_inicio,
            fecha_fin,
        }
    }
    pub fn get_id_membresia(&self) -> &str {
        &self.id_membresia
    }
    pub fn get_tipo(&self) -> String {
        self.tipo.clone()
    }
    pub fn get_estado(&self) -> Estado {
        self.estado.clone()
    }
    pub fn get_fecha_inicio(&self) -> NaiveDate {
        self.fecha_inicio.clone()
    }
    pub fn get_fecha_fin(&self) -> Option<NaiveDate> {
        self.fecha_fin
    }
    pub fn validate_membresia(&self) -> Result<(), ApiError> {
        if self.fecha_inicio > self.fecha_fin.unwrap_or_default() {
            return Err(ApiError::BadRequest(
                "Fecha de inicio no puede ser posterior a la fecha de fin".to_string(),
            ));
        }
        if self.fecha_inicio < NaiveDate::from_ymd_opt(1900, 1, 1).unwrap_or_default() {
            return Err(ApiError::BadRequest(
                "Fecha de inicio no puede ser anterior a 1900-01-01".to_string(),
            ));
        }
        if self.id_membresia.is_empty() {
            return Err(ApiError::BadRequest(
                "id_membresia no puede estar vacío".to_string(),
            ));
        }

        Ok(())
    }
}

impl From<CreateMembresiaRequest> for Membresia {
    fn from(request: CreateMembresiaRequest) -> Self {
        Self {
            id_membresia: Uuid::new_v4().to_string(),
            tipo: request.tipo,
            estado: request.estado,
            fecha_inicio: request.fecha_inicio,
            fecha_fin: None,
        }
    }
}
