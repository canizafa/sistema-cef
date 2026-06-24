use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::domain::*;

#[derive(Deserialize)]
pub struct EstadisticaRequest {
    pub fecha_desde: NaiveDate,
    pub fecha_hasta: NaiveDate,
}

#[derive(Serialize)]
pub struct ClaseMasConcurridaResponse {
    pub id_clase: String,
    pub descripcion: String,
    pub cantidad: i64,
}

impl From<ClaseMasConcurrida> for ClaseMasConcurridaResponse {
    fn from(value: ClaseMasConcurrida) -> Self {
        Self {
            id_clase: value.id_clase().to_string(),
            descripcion: value.descripcion().to_string(),
            cantidad: value.cantidad(),
        }
    }
}

#[derive(Serialize)]
pub struct ClaseMasCanceladaResponse {
    pub id_clase: String,
    pub descripcion: String,
    pub cantidad: i64,
}

impl From<ClaseMasCancelada> for ClaseMasCanceladaResponse {
    fn from(value: ClaseMasCancelada) -> Self {
        Self {
            id_clase: value.id_clase().to_string(),
            descripcion: value.descripcion().to_string(),
            cantidad: value.cantidad(),
        }
    }
}

#[derive(Serialize)]
pub struct RecaudacionResponse {
    pub total: f64,
}

impl From<Recaudacion> for RecaudacionResponse {
    fn from(value: Recaudacion) -> Self {
        Self {
            total: value.total(),
        }
    }
}
