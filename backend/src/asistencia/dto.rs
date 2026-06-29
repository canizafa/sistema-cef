use super::domain::Asistencia;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

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
            id_asistencia: asistencia.get_id_asistencia().to_string(),
            fecha: asistencia.get_fecha().clone(),
            metodo: asistencia.get_metodo().to_string(),
            id_reserva: asistencia.get_id_reserva().to_string(),
        }
    }
}
