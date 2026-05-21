use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CrearListaEspera {
    pub id_espera: i32,
    pub dni_cliente: i32,
    pub id_clase: i32,
    pub fecha: String,
}
