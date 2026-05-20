use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CrearReserva {
    pub fecha: String,
    pub estado: String,
    pub dni_cliente: i32,
    pub id_clase: i32,
}
