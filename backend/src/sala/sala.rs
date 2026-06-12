use super::*;

#[derive(Debug, Clone)]
pub struct Sala {
    pub id: String,
    pub numero: i64,
    pub capacidad_maxima: i64,
}

impl Sala {
    pub fn new(id: String, numero: i64, capacidad_maxima: i64) -> Self {
        Self {
            id,
            numero,
            capacidad_maxima,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_numero(&self) -> i64 {
        self.numero
    }
    pub fn get_capacidad_maxima(&self) -> i64 {
        self.capacidad_maxima
    }
    // Domain no debe conocer Apierror
    pub fn validate_sala(&self) -> Result<(), ApiError> {
        if self.numero <= 0 || self.capacidad_maxima <= 0 {
            Err(ApiError::BadRequest(
                "El número y la capacidad máxima deben ser mayores a 0".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}
impl From<CreateSalaRequest> for Sala {
    fn from(request: CreateSalaRequest) -> Self {
        Self::new(
            uuid::Uuid::new_v4().to_string(),
            request.numero,
            request.capacidad_maxima,
        )
    }
}
