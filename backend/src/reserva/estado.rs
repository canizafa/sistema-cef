use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EstadoReserva {
    Pendiente,
    Confirmada,
    Cancelada,
}

impl From<String> for EstadoReserva {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pendiente" => Self::Pendiente,
            "confirmada" => Self::Confirmada,
            "cancelada" => Self::Cancelada,
            _ => Self::Pendiente,
        }
    }
}

impl ToString for EstadoReserva {
    fn to_string(&self) -> String {
        match self {
            Self::Pendiente => "pendiente".to_string(),
            Self::Confirmada => "confirmada".to_string(),
            Self::Cancelada => "cancelada".to_string(),
        }
    }
}
