use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Rol {
    Profesor,
    Empleado,
    Cliente,
    Duenio,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Estado {
    Alta,
    Baja,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Genero {
    Masculino,
    Femenino,
    Otro,
}
