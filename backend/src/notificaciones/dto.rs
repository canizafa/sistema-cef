use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotificacionRequest {
    pub email: String,
    pub motivo: String,
    pub cuerpo: String,
}

#[derive(Debug, Deserialize)]
pub struct NotificacionUpdateRequest {
    pub fecha: NaiveDate,
}
