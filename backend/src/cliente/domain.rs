use super::{dto::CreateClienteRequest, errors::ClienteDomainError};
use crate::{
    app::rol::{Estado, Rol},
    cliente::dto::ClienteRequest,
};

use chrono::{Datelike, Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct Cliente {
    dni: i64,
    nombre_apellido: String,
    password_hash: String,
    email: String,
    telefono: String,
    fecha_nacimiento: NaiveDate,
    estado: Estado,
    id_ficha: String,
    rol: Rol,
}

impl Cliente {
    pub fn new(
        dni: i64,
        nombre_apellido: String,
        password_hash: String,
        email: String,
        telefono: String,
        fecha_nacimiento: NaiveDate,
        estado: Estado,
        id_ficha: String,
        rol: Rol,
    ) -> Self {
        Self {
            dni,
            nombre_apellido,
            password_hash,
            email,
            telefono,
            fecha_nacimiento,
            estado,
            id_ficha,
            rol,
        }
    }
    pub fn get_dni(&self) -> i64 {
        self.dni
    }
    pub fn get_nombre_apellido(&self) -> String {
        self.nombre_apellido.clone()
    }
    pub fn get_mail(&self) -> String {
        self.email.clone()
    }
    pub fn get_telefono(&self) -> String {
        self.telefono.clone()
    }
    pub fn get_fecha_nacimiento(&self) -> NaiveDate {
        self.fecha_nacimiento.clone()
    }
    pub fn get_estado(&self) -> Estado {
        self.estado.clone()
    }
    pub fn get_id_ficha(&self) -> &str {
        &self.id_ficha
    }
    pub fn get_rol(&self) -> Rol {
        self.rol.clone()
    }
    pub fn get_password_hash(&self) -> &str {
        &self.password_hash
    }
    pub fn update_password(
        &mut self,
        password_verificada: bool,
        new_password: &str,
    ) -> Vec<ClienteDomainError> {
        let mut vec_err = Vec::new();
        if new_password.len() < 8 {
            vec_err.push(ClienteDomainError::WeakPassword);
        }
        if !password_verificada {
            vec_err.push(ClienteDomainError::InvalidPassword);
        }
        if !vec_err.is_empty() {
            vec_err
        } else {
            self.password_hash = new_password.to_string();
            vec_err
        }
    }
    pub fn reset_password(&mut self, new_password: &str) {
        self.password_hash = new_password.to_string();
    }
    pub fn update_cliente(&mut self, nombre: &str, apellido: &str) -> Vec<ClienteDomainError> {
        let mut vec_err = Vec::new();
        if nombre.is_empty() {
            vec_err.push(ClienteDomainError::InvalidName);
        }
        if apellido.is_empty() {
            vec_err.push(ClienteDomainError::InvalidName);
        }
        if !vec_err.is_empty() {
            return vec_err;
        }
        self.nombre_apellido = format!("{}, {}", nombre, apellido);
        vec_err
    }
    pub fn validate_password_format(password: &str) -> Result<(), ClienteDomainError> {
        if password.len() < 8 {
            return Err(ClienteDomainError::WeakPassword);
        }
        Ok(())
    }
    pub fn validate_cliente(&self) -> Vec<ClienteDomainError> {
        let mut vec_err = Vec::new();
        if self.dni <= 0 {
            vec_err.push(ClienteDomainError::InvalidDni);
        }
        if self.nombre_apellido.is_empty() {
            vec_err.push(ClienteDomainError::InvalidName);
        }
        if let Err(err) = Self::validate_password_format(&self.password_hash) {
            vec_err.push(err);
        }
        if self.email.is_empty() {
            vec_err.push(ClienteDomainError::InvalidEmail);
        }
        if self.telefono.is_empty() {
            vec_err.push(ClienteDomainError::InvalidPhone);
        }
        let now = Local::now().date_naive();
        if now.year() - self.fecha_nacimiento.year() < 14 {
            vec_err.push(ClienteDomainError::InvalidBirthDate);
        }
        vec_err
    }
}

impl TryFrom<(CreateClienteRequest, String, String)> for Cliente {
    type Error = Vec<ClienteDomainError>;

    fn try_from(
        (request, password_hash, id_ficha): (CreateClienteRequest, String, String),
    ) -> Result<Self, Self::Error> {
        let cliente = Self {
            dni: request.dni,
            nombre_apellido: request.nombre_apellido,
            password_hash,
            email: request.email,
            telefono: request.telefono,
            fecha_nacimiento: request.fecha_nacimiento,
            estado: request.estado,
            id_ficha,
            rol: Rol::Cliente,
        };
        let errors = cliente.validate_cliente();
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(cliente)
    }
}
impl TryFrom<ClienteRequest> for Cliente {
    type Error = Vec<ClienteDomainError>;

    fn try_from(request: ClienteRequest) -> Result<Self, Self::Error> {
        let cliente = Self {
            dni: request.dni,
            nombre_apellido: request.nombre_apellido,
            password_hash: String::new(),
            email: request.email,
            telefono: request.telefono,
            fecha_nacimiento: request.fecha_nacimiento,
            estado: request.estado,
            id_ficha: request.id_ficha,
            rol: Rol::Cliente,
        };
        let errors = cliente.validate_cliente();
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(cliente)
    }
}
