use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotificacionRequest {
    pub mail: String,
    pub motivo: String,
    pub cuerpo: String,
}
