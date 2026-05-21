use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Cliente {
    pub dni: i32,
    pub nombre: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: String,
    pub estado: String,
    pub ficha: String,
}
