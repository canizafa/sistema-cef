use crate::{
    app::rol::{Estado, Genero},
    profesor::domain::Profesor,
};
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

#[derive(Debug, Deserialize)]
pub struct EliminarProfesorRequest {
    pub profesor_dni: i64,
    pub motivo_eliminacion: String,
}

impl From<Profesor> for ProfesorResponse {
    fn from(profesor: Profesor) -> Self {
        Self {
            dni: profesor.get_dni(),
            nombre_completo: profesor.get_nombre_completo().to_owned(),
            genero: profesor.get_genero().to_owned(),
            estado: profesor.get_estado().to_owned(),
        }
    }
}
