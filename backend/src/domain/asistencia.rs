use chrono::{Local, NaiveDate};

use crate::{domain::Cliente, dtos::CreateAsistenciaRequest, errors::ApiError};

pub struct Asistencia {
    pub id_asistencia: String,
    pub fecha: NaiveDate,
    pub metodo: String,
    pub id_clase: String,
    pub lista_espera: Vec<Cliente>,
}

impl Asistencia {
    pub fn get_id_asistencia(&self) -> String {
        self.id_asistencia.clone()
    }

    pub fn get_fecha(&self) -> NaiveDate {
        self.fecha.clone()
    }

    pub fn get_metodo(&self) -> String {
        self.metodo.clone()
    }

    pub fn get_id_clase(&self) -> String {
        self.id_clase.clone()
    }

    pub fn get_lista_espera(&self) -> Vec<Cliente> {
        self.lista_espera.clone()
    }
    pub fn validate_asistencia(&self) -> Result<(), ApiError> {
        if self.fecha > Local::now().naive_local().date() {
            return Err(ApiError::InvalidAsistencia);
        }
        if self.metodo.is_empty() {
            return Err(ApiError::InvalidAsistencia);
        }
        Ok(())
    }
}

impl From<CreateAsistenciaRequest> for Asistencia {
    fn from(request: CreateAsistenciaRequest) -> Self {
        Self {
            id_asistencia: uuid::Uuid::new_v4().to_string(),
            fecha: request.fecha,
            metodo: request.metodo,
            id_clase: request.id_reserva,
            lista_espera: Vec::new(),
        }
    }
}
