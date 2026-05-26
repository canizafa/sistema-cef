use crate::domain::{Estado, Membresia};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMembresiaRequest {
    pub id_membresia: String,
    pub tipo: String,
    pub estado: Estado,
    pub fecha_inicio: NaiveDate,
    pub fecha_fin: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct MembresiaResponse {
    pub id_membresia: String,
    pub tipo: String,
    pub estado: Estado,
    pub fecha_inicio: NaiveDate,
    pub fecha_fin: Option<NaiveDate>,
}

impl From<Membresia> for MembresiaResponse {
    fn from(membresia: Membresia) -> Self {
        Self {
            id_membresia: membresia.get_id_membresia().to_string(),
            tipo: membresia.get_tipo().to_string(),
            estado: membresia.get_estado(),
            fecha_inicio: membresia.get_fecha_inicio(),
            fecha_fin: membresia.get_fecha_fin(),
        }
    }
}
