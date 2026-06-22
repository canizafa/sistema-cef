use super::dto::CreateListaEsperaRequest;
use crate::app::errors::FieldError;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ListaEspera {
    id_espera: String,
    tipo: String,
    id_clase: String,
}
impl ListaEspera {
    pub fn new(id_espera: String, tipo: String, id_clase: String) -> Self {
        Self {
            id_espera,
            tipo,
            id_clase,
        }
    }
    pub fn get_id_lista(&self) -> &str {
        &self.id_espera
    }
    pub fn get_tipo(&self) -> &str {
        &self.tipo
    }
    pub fn get_id_clase(&self) -> &str {
        &self.id_clase
    }
    pub fn validate(&self) -> Vec<FieldError> {
        let mut errors = vec![];

        if self.tipo.trim().is_empty() {
            errors.push(FieldError::new("tipo", "Tipo invalido"));
        }

        if self.id_clase.trim().is_empty() {
            errors.push(FieldError::new("id_clase", "Clase invalida"));
        }
        errors
    }
}

impl From<CreateListaEsperaRequest> for ListaEspera {
    fn from(request: CreateListaEsperaRequest) -> Self {
        Self {
            id_espera: uuid::Uuid::new_v4().to_string(),
            tipo: request.tipo,
            id_clase: request.id_clase,
        }
    }
}
