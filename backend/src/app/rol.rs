use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Estado {
    // Alta,
    // Baja,
    // Eliminado,
    SinCupo,
    Extendido,
}

impl ToString for Estado {
    fn to_string(&self) -> String {
        match self {
            Estado::Eliminado => "eliminado".to_string(),
            Estado::SinCupo => "sin_cupo".to_string(),
            Estado::Alta => "alta".to_string(),
            Estado::Baja => "baja".to_string(),
            _ => panic!("Estado no válido"),
        }
    }
}
