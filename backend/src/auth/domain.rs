use crate::auth::dto::RegisterRequest;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Register {
    pub dni: String,
    pub nombre_apellido: String,
    pub password: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
}

impl Register {
    pub fn new(
        dni: String,
        nombre_apellido: String,
        password: String,
        email: String,
        telefono: String,
        fecha_nacimiento: NaiveDate,
    ) -> Self {
        Self {
            dni,
            nombre_apellido,
            password,
            email,
            telefono,
            fecha_nacimiento,
        }
    }

    pub fn get_dni(&self) -> String {
        self.dni.clone()
    }

    pub fn get_nombre_apellido(&self) -> String {
        self.nombre_apellido.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
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
}

impl From<RegisterRequest> for Register {
    fn from(request: RegisterRequest) -> Self {
        Self {
            dni: request.dni.to_string(),
            nombre_apellido: request.nombre_apellido,
            password: request.password,
            email: request.email,
            telefono: request.telefono,
            fecha_nacimiento: request.fecha_nacimiento,
        }
    }
}
