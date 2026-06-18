use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum GeneroUsuario {
    Masculino,
    Femenino,
    Otro,
}
impl From<String> for GeneroUsuario {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "masculino" => GeneroUsuario::Masculino,
            "femenino" => GeneroUsuario::Femenino,
            "otro" => GeneroUsuario::Otro,
            _ => GeneroUsuario::Masculino,
        }
    }
}
impl ToString for GeneroUsuario {
    fn to_string(&self) -> String {
        match self {
            GeneroUsuario::Masculino => "masculino".to_string(),
            GeneroUsuario::Femenino => "femenino".to_string(),
            GeneroUsuario::Otro => "otro".to_string(),
        }
    }
}
