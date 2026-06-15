use crate::{
    app::rol::Estado,
    membresia::{dto::CreateMembresiaRequest, errors::MembresiaDomainError},
};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Membresia {
    id_membresia: String,
    tipo_actividad: String,
    dni_cliente: i64,
    estado: Estado,
    fecha_inicio: NaiveDate,
    fecha_fin: Option<NaiveDate>,
}

impl Membresia {
    pub fn new(
        id_membresia: String,
        tipo_actividad: String,
        dni_cliente: i64,
        estado: Estado,
        fecha_inicio: NaiveDate,
        fecha_fin: Option<NaiveDate>,
    ) -> Self {
        Self {
            id_membresia,
            tipo_actividad,
            dni_cliente,
            estado,
            fecha_inicio,
            fecha_fin,
        }
    }
    pub fn get_id_membresia(&self) -> &str {
        &self.id_membresia
    }
    pub fn get_tipo_actividad(&self) -> String {
        self.tipo_actividad.clone()
    }
    pub fn get_dni_cliente(&self) -> i64 {
        self.dni_cliente
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
    // Domain no debe conocer apierror
    pub fn validate_membresia(&self) -> Vec<MembresiaDomainError> {
        let mut vec_errors = Vec::new();
        if self.fecha_inicio > self.fecha_fin.unwrap_or_default() {
            vec_errors.push(MembresiaDomainError::FechaInicioPosteriorAFechaFin);
        }
        if self.fecha_inicio < NaiveDate::from_ymd_opt(1900, 1, 1).unwrap_or_default() {
            vec_errors.push(MembresiaDomainError::FechaInicioAnteriorA1900);
        }
        if self.id_membresia.is_empty() {
            vec_errors.push(MembresiaDomainError::IdMembresiaVacio);
        }

        vec_errors
    }
}

impl From<CreateMembresiaRequest> for Membresia {
    fn from(request: CreateMembresiaRequest) -> Self {
        Self {
            id_membresia: Uuid::new_v4().to_string(),
            tipo_actividad: request.tipo,
            dni_cliente: request.dni_cliente,
            estado: request.estado,
            fecha_inicio: request.fecha_inicio,
            fecha_fin: None,
        }
    }
}
