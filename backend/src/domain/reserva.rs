use chrono::NaiveDate;

use crate::domain::Estado;

pub struct Reserva {
    id_reserva: i32,
    estado: Estado,
    tipo: String,
    fecha_reserva: NaiveDate,
    id_clase: i32,
}

impl Reserva {
    pub fn get_id(&self) -> i32 {
        self.id_reserva
    }

    pub fn get_estado(&self) -> &Estado {
        &self.estado
    }

    pub fn get_tipo(&self) -> &str {
        &self.tipo
    }

    pub fn get_fecha_reserva(&self) -> NaiveDate {
        self.fecha_reserva
    }

    pub fn get_id_clase(&self) -> i32 {
        self.id_clase
    }
}
