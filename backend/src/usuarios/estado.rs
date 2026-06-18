use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EstadoUsuario {
    Alta,
    Baja,
    Eliminado,
}

impl From<String> for EstadoUsuario {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "eliminado" => EstadoUsuario::Eliminado,
            "alta" => EstadoUsuario::Alta,
            "baja" => EstadoUsuario::Baja,
            _ => panic!("Estado no válido"),
        }
    }
}

impl ToString for EstadoUsuario {
    fn to_string(&self) -> String {
        match self {
            EstadoUsuario::Alta => "alta".to_string(),
            EstadoUsuario::Baja => "baja".to_string(),
            EstadoUsuario::Eliminado => "eliminado".to_string(),
        }
    }
}
