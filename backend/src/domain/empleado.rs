use crate::{
    auth::password::hash_password, domain::Rol, dtos::CreateEmpleadoRequest, errors::ApiError,
};

#[derive(Debug, Clone)]
pub struct Empleado {
    pub dni_empleado: i64,
    pub nombre_apellido: String,
    pub password_hash: String,
    pub mail: String,
    pub password: String,
    pub genero: String,
    pub estado: String,
    pub rol: Rol,
}

impl Empleado {
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
    pub fn get_estado(&self) -> String {
        self.estado.clone()
    }
    pub fn get_rol(&self) -> Rol {
        self.rol.clone()
    }
    pub fn get_password_hash(&self) -> String {
        self.password_hash.clone()
    }
    pub fn update_password(&mut self, password_hash: &str) -> Result<(), ApiError> {
        self.password_hash =
            hash_password(password_hash).map_err(|_| ApiError::InternalServerError)?;
        Ok(())
    }
}

impl From<CreateEmpleadoRequest> for Empleado {
    fn from(request: CreateEmpleadoRequest) -> Self {
        let password_hash = hash_password(&request.password).expect("Failed to hash password");
        Self {
            dni_empleado: request.dni,
            nombre_apellido: request.nombre_apellido,
            password_hash,
            mail: request.mail,
            password: request.password,
            genero: request.genero,
            estado: request.estado,
            rol: request.rol,
        }
    }
}
