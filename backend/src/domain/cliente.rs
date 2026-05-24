use chrono::NaiveDate;

use crate::{
    domain::{Estado, FichaMedica, Rol},
    dtos::CreateClienteRequest,
};

#[derive(Debug, Clone)]
pub struct Cliente {
    dni: String,
    nombre_apellido: String,
    password: String,
    email: String,
    telefono: String,
    fecha_nacimiento: NaiveDate,
    estado: Estado,
    ficha_medica: FichaMedica,
    rol: Rol,
}

impl Cliente {
    pub fn get_dni(&self) -> String {
        self.dni.clone()
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
        self.password.clone()
    }
}

impl From<CreateClienteRequest> for Cliente {
    fn from(request: CreateClienteRequest) -> Self {
        Self {
            dni: request.dni,
            nombre_apellido: request.nombre_apellido,
            password: request.password,
            email: request.email,
            telefono: request.telefono,
            fecha_nacimiento: request.fecha_nacimiento,
            estado: request.estado,
            ficha_medica: request.ficha_medica.into(),
            rol: Rol::Cliente,
        }
    }
}
