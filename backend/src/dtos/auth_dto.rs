use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::rol::Rol;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub dni: i32,
    pub nombre_apellido: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub id_usuario: i32,
    pub email: String,
    pub access_token: String,
    pub rol: Rol,
}
