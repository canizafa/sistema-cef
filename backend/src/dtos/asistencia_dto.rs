use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::asistencia::Asistencia;

#[derive(Debug, Deserialize)]
pub struct CreateAsistenciaRequest {
    pub fecha: NaiveDate,
    pub metodo: String,
    pub id_reserva: String,
}

#[derive(Debug, Serialize)]
pub struct AsistenciaResponse {
    pub id_asistencia: String,
    pub fecha: NaiveDate,
    pub metodo: String,
    pub id_reserva: String,
}

#[derive(Debug, Serialize)]
pub struct AsistenciaListResponse {
    pub asistencias: Vec<AsistenciaResponse>,
}

impl From<Asistencia> for AsistenciaResponse {
    fn from(asistencia: Asistencia) -> Self {
        Self {
            id_asistencia: asistencia.get_id_asistencia(),
            fecha: asistencia.get_fecha(),
            metodo: asistencia.get_metodo(),
            id_reserva: asistencia.get_id_clase(),
        }
    }
}
