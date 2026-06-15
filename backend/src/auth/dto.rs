use crate::app::rol::Rol;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateChangePasswordRequest {
    pub dni_cliente: i64,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub dni: i64,
    pub nombre_apellido: String,
    pub password: String,
    pub email: String,
    pub telefono: String,
    pub rol: Rol,
    pub fecha_nacimiento: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub dni: i64,
    pub email: String,
    pub access_token: String,
    pub rol: Rol,
}
