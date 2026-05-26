use serde::{Deserialize, Serialize};

use crate::domain::sala::Sala;

#[derive(Debug, Deserialize)]
pub struct CreateSalaRequest {
    pub numero: i64,
    pub capacidad_maxima: i64,
}

#[derive(Debug, Serialize)]
pub struct SalaResponse {
    pub id: String,
    pub numero: i64,
    pub capacidad_maxima: i64,
}

impl From<Sala> for SalaResponse {
    fn from(sala: Sala) -> Self {
        Self {
            id: sala.get_id().to_string(),
            numero: sala.get_numero(),
            capacidad_maxima: sala.get_capacidad_maxima(),
        }
    }
}
