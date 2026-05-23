use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateAsistenciaRequest {
    fecha: NaiveDate,
    metodo: String,
    id_reserva: i32,
}

#[derive(Debug, Serialize)]
pub struct AsistenciaResponse {
    id_asistencia: i32,
    fecha: NaiveDate,
    metodo: String,
    id_reserva: i32,
}

#[derive(Debug, Serialize)]
pub struct AsistenciaListResponse {
    pub asistencias: Vec<AsistenciaResponse>,
}
