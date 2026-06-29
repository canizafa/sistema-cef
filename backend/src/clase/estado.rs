use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EstadoClase {
    Alta,
    SinCupo,
    Extendido,
}

impl From<String> for EstadoClase {
    fn from(s: String) -> Self {
        match s.as_str() {
            "alta" => EstadoClase::Alta,
            "sin_cupo" => EstadoClase::SinCupo,
            "extendido" => EstadoClase::Extendido,
            _ => panic!("Estado no permitido"),
        }
    }
}

impl ToString for EstadoClase {
    fn to_string(&self) -> String {
        match self {
            EstadoClase::Alta => "alta".to_string(),
            EstadoClase::SinCupo => "sin_cupo".to_string(),
            EstadoClase::Extendido => "extendido".to_string(),
        }
    }
}
