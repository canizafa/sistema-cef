use crate::domain::Rol;

pub struct Empleado {
    pub dni: i32,
    pub nombre_apellido: String,
    pub mail: String,
    pub genero: String,
    pub estado: String,
    pub rol: Rol,
}

impl Empleado {
    pub fn get_dni(&self) -> i32 {
        self.dni
    }
    pub fn get_nombre_apellido(&self) -> &str {
        &self.nombre_apellido
    }
    pub fn get_mail(&self) -> &str {
        &self.mail
    }
    pub fn get_genero(&self) -> &str {
        &self.genero
    }
    pub fn get_estado(&self) -> &str {
        &self.estado
    }
    pub fn get_rol(&self) -> &Rol {
        &self.rol
    }
}
