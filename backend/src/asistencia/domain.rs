use crate::asistencia::{dto::CreateAsistenciaRequest, errors::AsistenciaDomainError};
use chrono::{Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct Asistencia {
    pub id_asistencia: String,
    pub fecha: NaiveDate,
    pub metodo: String,
    pub id_reserva: String,
}

impl Asistencia {
    pub fn new(
        id_asistencia: String,
        fecha: NaiveDate,
        metodo: String,
        id_reserva: String,
    ) -> Self {
        Self {
            id_asistencia,
            fecha,
            metodo,
            id_reserva,
        }
    }

    pub fn get_id_asistencia(&self) -> &str {
        &self.id_asistencia
    }

    pub fn get_fecha(&self) -> &NaiveDate {
        &self.fecha
    }

    pub fn get_metodo(&self) -> &str {
        &self.metodo
    }

    pub fn get_id_reserva(&self) -> &str {
        &self.id_reserva
    }
    //refactorizar, domain no tendría que saber de api error?
    pub fn validate_asistencia(&self) -> Result<(), AsistenciaDomainError> {
        if self.fecha > Local::now().naive_local().date() {
            return Err(AsistenciaDomainError::InvalidAsistencia);
        }
        if self.metodo.is_empty() {
            return Err(AsistenciaDomainError::InvalidAsistencia);
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
            id_reserva: request.id_reserva,
        }
    }
}
