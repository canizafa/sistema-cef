use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Cliente {
    pub dni: i32,
    pub nombre: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: String,
    pub estado: String,
    pub ficha: String,
}
