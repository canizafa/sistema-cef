use super::*;
#[derive(Debug, Clone)]
pub struct Actividad {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
}

impl Actividad {
    pub fn new(id: String, nombre: String, descripcion: String) -> Self {
        Self {
            id,
            nombre,
            descripcion,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_nombre(&self) -> &str {
        &self.nombre
    }
    pub fn get_descripcion(&self) -> &str {
        &self.descripcion
    }
    pub fn update_actividad(&mut self, other: Self) {
        self.nombre = other.nombre;
        self.descripcion = other.descripcion;
    }
    pub fn validate_actividad(&self) -> Vec<ActividadDomainError> {
        let mut vec_err = Vec::new();
        if self.nombre.is_empty() {
            vec_err.push(ActividadDomainError::NombreVacio);
        }
        if self.descripcion.is_empty() {
            vec_err.push(ActividadDomainError::DescripcionVacia);
        }
        vec_err
    }
}

impl From<CreateActividadRequest> for Actividad {
    fn from(value: CreateActividadRequest) -> Self {
        Self::new(
            uuid::Uuid::new_v4().to_string(),
            value.nombre,
            value.descripcion,
        )
    }
}
