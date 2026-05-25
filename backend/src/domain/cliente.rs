use chrono::{Datelike, Local, NaiveDate};

use crate::{
    auth::password::hash_password,
    domain::{Estado, FichaMedica, Rol},
    dtos::CreateClienteRequest,
    errors::ApiError,
};

#[derive(Debug, Clone)]
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
    pub fn update_password(&mut self, password: &str) -> Result<(), ApiError> {
        if password.len() < 8 {
            return Err(ApiError::WeakPassword);
        }
        self.password_hash = hash_password(password)?;
        Ok(())
    }
    pub fn validate_cliente(&self) -> Result<(), ApiError> {
        if self.dni <= 0 {
            return Err(ApiError::InvalidDni);
        }
        if self.nombre_apellido.is_empty() {
            return Err(ApiError::InvalidName);
        }
        if self.email.is_empty() {
            return Err(ApiError::InvalidEmail);
        }
        if self.telefono.is_empty() {
            return Err(ApiError::InvalidPhone);
        }
        let now = Local::now().date_naive();
        if now.year() - self.fecha_nacimiento.year() < 14 {
            return Err(ApiError::InvalidBirthDate);
        }
        Ok(())
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
