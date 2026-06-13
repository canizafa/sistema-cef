use super::{
    dto::{CreateClienteRequest, UpdateClienteRequest},
    errors::ClienteDomainError,
};
use crate::app::rol::{Estado, Rol};
use crate::auth::password::hash_password;
use crate::ficha_medica::domain::FichaMedica;
use chrono::{Datelike, Local, NaiveDate};

#[derive(Debug)]
pub struct Cliente {
    dni: i64,
    nombre_apellido: String,
    password_hash: String,
    email: String,
    telefono: String,
    fecha_nacimiento: NaiveDate,
    estado: Estado,
    ficha_medica: FichaMedica,
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
        ficha_medica: FichaMedica,
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
            ficha_medica,
            rol,
        }
    }
    pub fn get_dni(&self) -> i64 {
        self.dni
    }
    pub fn get_nombre_apellido(&self) -> String {
        self.nombre_apellido.clone()
    }
    pub fn get_email(&self) -> String {
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
    pub fn get_ficha_medica(&self) -> FichaMedica {
        self.ficha_medica.clone()
    }
    pub fn get_rol(&self) -> Rol {
        self.rol.clone()
    }
    pub fn get_password_hash(&self) -> String {
        self.password_hash.clone()
    }
    pub fn update_password(&mut self, password: &str) -> Result<(), ClienteDomainError> {
        if password.len() < 8 {
            return Err(ClienteDomainError::WeakPassword);
        }
        self.password_hash = hash_password(password)?;
        Ok(())
    }
    pub fn update_cliente(&mut self, other: Self) -> Result<(), ClienteDomainError> {
        todo!()
    }
    pub fn validate_cliente(&self) -> Vec<ClienteDomainError> {
        let mut vec_err = Vec::new();
        if self.dni <= 0 {
            vec_err.push(ClienteDomainError::InvalidDni);
        }
        if self.nombre_apellido.is_empty() {
            vec_err.push(ClienteDomainError::InvalidName);
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

impl From<CreateClienteRequest> for Cliente {
    fn from(request: CreateClienteRequest) -> Self {
        let password_hash = hash_password(&request.password).expect("Contraseña no hasheada");
        Self {
            dni: request.dni,
            nombre_apellido: request.nombre_apellido,
            password_hash,
            email: request.email,
            telefono: request.telefono,
            fecha_nacimiento: request.fecha_nacimiento,
            estado: request.estado,
            ficha_medica: request.ficha_medica.into(),
            rol: Rol::Cliente,
        }
    }
}

impl From<UpdateClienteRequest> for Cliente {
    fn from(request: UpdateClienteRequest) -> Self {
        Self {
            dni: request.dni,
            nombre_apellido: request.nombre_apellido,
            password_hash: "123456".to_string(),
            email: request.email,
            telefono: request.telefono,
            fecha_nacimiento: request.fecha_nacimiento,
            estado: request.estado,
            ficha_medica: request.ficha_medica.into(),
            rol: Rol::Cliente,
        }
    }
}
