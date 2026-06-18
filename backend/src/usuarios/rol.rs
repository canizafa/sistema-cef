use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum RolUsuario {
    Empleado,
    Cliente,
    Duenio,
}

impl From<String> for RolUsuario {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "empleado" => RolUsuario::Empleado,
            "cliente" => RolUsuario::Cliente,
            "duenio" => RolUsuario::Duenio,
            _ => panic!("Rol no permitido"),
        }
    }
}

impl ToString for RolUsuario {
    fn to_string(&self) -> String {
        match self {
            RolUsuario::Empleado => "empleado".to_string(),
            RolUsuario::Cliente => "cliente".to_string(),
            RolUsuario::Duenio => "duenio".to_string(),
        }
    }
}
