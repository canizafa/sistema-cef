use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Rol {
    Profesor,
    Empleado,
    Cliente,
    Duenio,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Estado {
    Alta,
    Baja,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Genero {
    Masculino,
    Femenino,
    Otro,
}

impl From<String> for Estado {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "alta" => Estado::Alta,
            "baja" => Estado::Baja,
            _ => Estado::Alta,
        }
    }
}

impl From<String> for Genero {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "masculino" => Genero::Masculino,
            "femenino" => Genero::Femenino,
            "otro" => Genero::Otro,
            _ => Genero::Masculino,
        }
    }
}

impl From<String> for Rol {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "profesor" => Rol::Profesor,
            "empleado" => Rol::Empleado,
            "cliente" => Rol::Cliente,
            "duenio" => Rol::Duenio,
            _ => Rol::Profesor,
        }
    }
}
