use super::{
    dto::{CreateEmpleadoRequest, UpdateEmpleadoRequest},
    errors::EmpleadoDomainError,
};
use crate::app::rol::{Estado, Rol};

#[derive(Debug)]
pub struct Empleado {
    dni_empleado: i64,
    nombre_apellido: String,
    password_hash: String,
    mail: String,
    genero: String,
    estado: Estado,
    rol: Rol,
    motivo_eliminacion: Option<String>,
}

impl Empleado {
    pub fn new(
        dni_empleado: i64,
        nombre_apellido: String,
        password_hash: String,
        mail: String,
        genero: String,
        estado: Estado,
        rol: Rol,
        motivo_eliminacion: Option<String>,
    ) -> Self {
        Self {
            dni_empleado,
            nombre_apellido,
            password_hash,
            mail,
            genero,
            estado,
            rol,
            motivo_eliminacion,
        }
    }
    pub fn get_dni(&self) -> i64 {
        self.dni_empleado
    }
    pub fn get_nombre_apellido(&self) -> String {
        self.nombre_apellido.clone()
    }
    pub fn get_mail(&self) -> String {
        self.mail.clone()
    }
    pub fn get_genero(&self) -> String {
        self.genero.clone()
    }
    pub fn get_estado(&self) -> Estado {
        self.estado.clone()
    }
    pub fn get_rol(&self) -> Rol {
        self.rol.clone()
    }
    pub fn get_password_hash(&self) -> String {
        self.password_hash.clone()
    }
    pub fn get_motivo_eliminacion(&self) -> Option<String> {
        self.motivo_eliminacion.clone()
    }
    pub fn update_password(&mut self, password_hash: &str) {
        self.password_hash = password_hash.to_string();
    }
    pub fn validate_empleado(&self) -> Vec<EmpleadoDomainError> {
        let mut errors = Vec::new();
        if self.dni_empleado <= 0 {
            errors.push(EmpleadoDomainError::InvalidDNI);
        }
        if !self.mail.contains('@') {
            errors.push(EmpleadoDomainError::InvalidEmail);
        }
        if self.password_hash.len() < 5 {
            errors.push(EmpleadoDomainError::WeakPassword);
        }
        errors
    }
}

impl From<(CreateEmpleadoRequest, String)> for Empleado {
    fn from((request, password_hash): (CreateEmpleadoRequest, String)) -> Self {
        Self {
            dni_empleado: request.dni,
            nombre_apellido: request.nombre_apellido,
            password_hash,
            mail: request.mail,
            genero: request.genero,
            estado: request.estado,
            rol: request.rol,
            motivo_eliminacion: None,
        }
    }
}
impl From<(UpdateEmpleadoRequest, String)> for Empleado {
    fn from((request, password_hash): (UpdateEmpleadoRequest, String)) -> Self {
        Self {
            dni_empleado: request.dni,
            nombre_apellido: request.nombre_apellido,
            password_hash,
            mail: request.mail,
            genero: request.genero,
            estado: request.estado,
            rol: request.rol,
            motivo_eliminacion: request.motivo_eliminacion,
        }
    }
}
