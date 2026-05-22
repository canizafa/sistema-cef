use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateFichaMedicaRequest {
    pub enfermedades: bool,
    pub operaciones_quirurgicas: bool,
    pub detalle: String,
}

#[derive(Debug, Serialize)]
pub struct FichaMedicaResponse {
    pub id_ficha: i32,
    pub enfermedades: bool,
    pub operaciones_quirurgicas: bool,
    pub detalle: String,
}
