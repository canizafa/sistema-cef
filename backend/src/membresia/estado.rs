use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EstadoMembresia {
    Activo,
    Cancelado,
}

impl From<String> for EstadoMembresia {
    fn from(value: String) -> Self {
        match value.as_str() {
            "cancelado" => Self::Cancelado,
            "activo" => Self::Activo,
            _ => Self::Activo,
        }
    }
}

impl ToString for EstadoMembresia {
    fn to_string(&self) -> String {
        match self {
            Self::Activo => "activo".to_string(),
            Self::Cancelado => "cancelado".to_string(),
        }
    }
}
