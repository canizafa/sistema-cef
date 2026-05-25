use crate::{
    domain::{Estado, Genero},
    dtos::CreateProfesorRequest,
    errors::ApiError,
};

pub struct Profesor {
    dni: i64,
    nombre_completo: String,
    genero: Genero,
    estado: Estado,
}

impl Profesor {
    pub fn get_dni(&self) -> i64 {
        self.dni
    }

    pub fn get_nombre_completo(&self) -> String {
        self.nombre_completo.clone()
    }

    pub fn get_genero(&self) -> Genero {
        self.genero.clone()
    }

    pub fn get_estado(&self) -> Estado {
        self.estado.clone()
    }
    pub fn validate_profesor(&self) -> Result<(), ApiError> {
        if self.dni <= 0 {
            return Err(ApiError::BadRequest(
                "dni must be greater than 0".to_string(),
            ));
        }
        if self.nombre_completo.is_empty() {
            return Err(ApiError::BadRequest(
                "nombre_completo cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

impl From<CreateProfesorRequest> for Profesor {
    fn from(value: CreateProfesorRequest) -> Self {
        Profesor {
            dni: value.dni,
            nombre_completo: value.nombre_completo,
            genero: value.genero,
            estado: value.estado,
        }
    }
}
