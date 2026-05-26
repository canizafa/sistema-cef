use crate::domain::{Estado, Membresia};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMembresiaRequest {
    pub id_membresia: String,
    pub tipo: String,
    pub estado: Estado,
    pub fecha_inicio: NaiveDate,
    pub fecha_fin: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct MembresiaResponse {
    pub id_membresia: String,
    pub tipo: String,
    pub estado: Estado,
    pub fecha_inicio: NaiveDate,
    pub fecha_fin: NaiveDate,
}

pub struct ListMembresiaResponse {
    pub membresias: Vec<MembresiaResponse>,
}

impl From<Membresia> for MembresiaResponse {
    fn from(membresia: Membresia) -> Self {
        Self {
            id_membresia: membresia.get_id_membresia().to_string(),
            tipo: membresia.get_tipo().to_string(),
            estado: membresia.get_estado(),
            fecha_inicio: membresia.get_fecha_inicio(),
            fecha_fin: membresia.get_fecha_fin().unwrap_or_default(),
        }
    }
}

impl From<Vec<Membresia>> for ListMembresiaResponse {
    fn from(membresias: Vec<Membresia>) -> Self {
        Self {
            membresias: membresias
                .into_iter()
                .map(MembresiaResponse::from)
                .collect(),
        }
    }
}
