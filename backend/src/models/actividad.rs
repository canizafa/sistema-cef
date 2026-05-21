use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct CrearActividad {
    pub nombre: String,
    pub descripcion: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Actividad {
    pub id: i32,
    pub nombre: String,
    pub descripcion: String,
}
