use super::domain::Cliente;
use crate::{
    ficha_medica::dto::CreateFichaMedicaRequest,
    usuarios::{estado::EstadoUsuario, rol::RolUsuario},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct CreateClienteRequest {
    pub dni: i64,
    pub nombre_apellido: String,
    pub password: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
    pub estado: EstadoUsuario,
    pub fecha_vencimiento: NaiveDate,
    pub ficha_medica: CreateFichaMedicaRequest,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePasswordRequest {
    pub email: String,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct ClienteRequest {
    pub dni: i64,
    pub nombre_apellido: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
    pub motivo_eliminacion: Option<String>,
    pub estado: EstadoUsuario,
    pub id_ficha: String,
}

#[derive(Debug, Deserialize)]
pub struct EliminarClienteRequest {
    pub dni: i64,
    pub estado: EstadoUsuario,
    pub motivo_eliminacion: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct UsarCreditosRequest {
    pub dni: i64,
    pub monto: i64,
}

#[derive(Debug, Serialize)]
pub struct ClienteResponse {
    pub dni: i64,
    pub nombre_apellido: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
    pub estado: EstadoUsuario,
    pub rol: RolUsuario,
    pub motivo_eliminacion: Option<String>,
    pub id_ficha: String,
    pub creditos: i64,
    pub fecha_notificacion: Option<NaiveDate>,
}
impl From<Cliente> for ClienteResponse {
    fn from(cliente: Cliente) -> Self {
        Self {
            dni: cliente.get_dni(),
            nombre_apellido: cliente.get_nombre_apellido(),
            email: cliente.get_mail(),
            telefono: cliente.get_telefono(),
            fecha_nacimiento: cliente.get_fecha_nacimiento(),
            estado: cliente.get_estado(),
            rol: cliente.get_rol(),
            motivo_eliminacion: cliente.get_motivo_eliminacion(),
            id_ficha: cliente.get_id_ficha().to_string(),
            creditos: cliente.get_creditos(),
            fecha_notificacion: cliente.get_fecha_notificacion(),
        }
    }
}
