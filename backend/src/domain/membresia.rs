use chrono::NaiveDate;

use crate::domain::Estado;

pub struct Membresia {
    id_membresia: i32,
    tipo: String,
    estado: Estado,
    fecha_inicio: NaiveDate,
    fecha_fin: NaiveDate,
}

impl Membresia {
    pub fn get_id_membresia(&self) -> i32 {
        self.id_membresia
    }
    pub fn get_tipo(&self) -> &str {
        &self.tipo
    }
    pub fn get_estado(&self) -> &Estado {
        &self.estado
    }
    pub fn get_fecha_inicio(&self) -> NaiveDate {
        self.fecha_inicio
    }
    pub fn get_fecha_fin(&self) -> NaiveDate {
        self.fecha_fin
    }
}
