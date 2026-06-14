use crate::{
    app::rol::{Estado, Genero},
    profesor::{dto::CreateProfesorRequest, errors::ProfesorDomainError},
};

#[derive(Debug)]
pub struct Profesor {
    dni: i64,
    nombre_completo: String,
    genero: Genero,
    estado: Estado,
}

impl Profesor {
    pub fn new(dni: i64, nombre_completo: String, genero: Genero, estado: Estado) -> Self {
        Self {
            dni,
            nombre_completo,
            genero,
            estado,
        }
    }
    pub fn get_dni(&self) -> i64 {
        self.dni
    }

    pub fn get_nombre_completo(&self) -> &str {
        &self.nombre_completo
    }

    pub fn get_genero(&self) -> &Genero {
        &self.genero
    }

    pub fn get_estado(&self) -> &Estado {
        &self.estado
    }
    pub fn validate_profesor(&self) -> Vec<ProfesorDomainError> {
        let mut errors = Vec::new();
        if self.dni <= 0 {
            errors.push(ProfesorDomainError::DniInvalid);
        }
        if self.nombre_completo.is_empty() {
            errors.push(ProfesorDomainError::NombreCompletoEmpty);
        }

        errors
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
