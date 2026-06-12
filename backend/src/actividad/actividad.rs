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
    //Domain no debe conocer appiError
    pub fn validate_actividad(&self) -> Result<(), ApiError> {
        if self.nombre.is_empty() || self.descripcion.is_empty() {
            Err(ApiError::BadRequest(
                "Nombre y descripción son obligatorios".to_string(),
            ))
        } else {
            Ok(())
        }
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
