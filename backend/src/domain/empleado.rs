use crate::{domain::Rol, dtos::CreateEmpleadoRequest};

#[derive(Debug, Clone)]
pub struct Empleado {
    pub dni: String,
    pub nombre_apellido: String,
    pub mail: String,
    pub genero: String,
    pub estado: String,
    pub rol: Rol,
}

impl Empleado {
    pub fn get_dni(&self) -> String {
        self.dni.clone()
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
}

impl From<CreateEmpleadoRequest> for Empleado {
    fn from(request: CreateEmpleadoRequest) -> Self {
        Self {
            dni: request.dni,
            nombre_apellido: request.nombre_apellido,
            mail: request.mail,
            genero: request.genero,
            estado: request.estado,
            rol: request.rol,
        }
    }
}
