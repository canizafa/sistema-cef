use serde::{Deserialize, Serialize};

use crate::domain::FichaMedica;

#[derive(Debug, Deserialize)]
pub struct CreateFichaMedicaRequest {
    pub enfermedades: bool,
    pub operaciones_quirurgicas: bool,
    pub detalle: String,
}

#[derive(Debug, Serialize)]
pub struct FichaMedicaResponse {
    pub id_ficha: String,
    pub enfermedades: bool,
    pub operaciones_quirurgicas: bool,
    pub detalle: String,
}

impl From<FichaMedica> for FichaMedicaResponse {
    fn from(ficha_medica: FichaMedica) -> Self {
        Self {
            id_ficha: ficha_medica.get_id_ficha(),
            enfermedades: ficha_medica.get_enfermedades(),
            operaciones_quirurgicas: ficha_medica.get_operaciones_quirurgicas(),
            detalle: ficha_medica.get_detalles(),
        }
    }
}
