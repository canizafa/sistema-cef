use chrono::NaiveDate;

use crate::{domain::Estado, dtos::CreateMembresiaRequest};

#[derive(Debug, Clone)]
pub struct Membresia {
    id_membresia: String,
    tipo: String,
    estado: Estado,
    fecha_inicio: NaiveDate,
    fecha_fin: Option<NaiveDate>,
}

impl Membresia {
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
}

impl From<CreateMembresiaRequest> for Membresia {
    fn from(request: CreateMembresiaRequest) -> Self {
        Self {
            id_membresia: request.id_membresia,
            tipo: request.tipo,
            estado: request.estado,
            fecha_inicio: request.fecha_inicio,
            fecha_fin: request.fecha_fin,
        }
    }
}
