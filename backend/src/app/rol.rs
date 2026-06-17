use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Rol {
    Empleado,
    Cliente,
    Duenio,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Estado {
    Alta,
    Baja,
    Eliminado,
    SinCupo,
    Extendido,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
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
            "sin_cupo" => Estado::SinCupo,
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
            "empleado" => Rol::Empleado,
            "cliente" => Rol::Cliente,
            "duenio" => Rol::Duenio,
            _ => {
                tracing::info!("Rol no válido");
                panic!("El rol enviado no existe")
            }
        }
    }
}

impl ToString for Rol {
    fn to_string(&self) -> String {
        match self {
            Rol::Empleado => "empleado".to_string(),
            Rol::Cliente => "cliente".to_string(),
            Rol::Duenio => "duenio".to_string(),
        }
    }
}

impl ToString for Estado {
    fn to_string(&self) -> String {
        match self {
            Estado::Eliminado => "eliminado".to_string(),
            Estado::SinCupo => "sin_cupo".to_string(),
            Estado::Alta => "alta".to_string(),
            Estado::Baja => "baja".to_string(),
        }
    }
}

impl ToString for Genero {
    fn to_string(&self) -> String {
        match self {
            Genero::Masculino => "masculino".to_string(),
            Genero::Femenino => "femenino".to_string(),
            Genero::Otro => "otro".to_string(),
        }
    }
}
