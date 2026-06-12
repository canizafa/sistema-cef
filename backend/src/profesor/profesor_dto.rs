use super::*;
use crate::app::{Estado, Genero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateProfesorRequest {
    pub dni: i64,
    pub nombre_completo: String,
    pub genero: Genero,
    pub estado: Estado,
}

#[derive(Debug, Serialize)]
pub struct ProfesorResponse {
    pub dni: i64,
    pub nombre_completo: String,
    pub genero: Genero,
    pub estado: Estado,
}

impl From<Profesor> for ProfesorResponse {
    fn from(profesor: Profesor) -> Self {
        Self {
            dni: profesor.get_dni(),
            nombre_completo: profesor.get_nombre_completo(),
            genero: profesor.get_genero(),
            estado: profesor.get_estado(),
        }
    }
}
