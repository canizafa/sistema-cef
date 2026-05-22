use chrono::NaiveDate;

use crate::domain::{Estado, FichaMedica};

pub struct Cliente {
    dni: i32,
    nombre_apellido: String,
    email: String,
    telefono: String,
    fecha_nacimiento: NaiveDate,
    estado: Estado,
    ficha_medica: FichaMedica,
}

impl Cliente {
    pub fn get_dni(&self) -> i32 {
        self.dni
    }
    pub fn get_nombre_apellido(&self) -> &str {
        &self.nombre_apellido
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn get_telefono(&self) -> &str {
        &self.telefono
    }
    pub fn get_fecha_nacimiento(&self) -> NaiveDate {
        self.fecha_nacimiento
    }
    pub fn get_estado(&self) -> &Estado {
        &self.estado
    }
    pub fn get_ficha_medica(&self) -> &FichaMedica {
        &self.ficha_medica
    }
}
