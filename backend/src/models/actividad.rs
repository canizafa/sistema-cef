use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CrearActividad {
    pub nombre: String,
    pub descripcion: String,
}

#[derive(Serialize, Deserialize)]
pub struct Actividad {
    pub id: i32,
    pub nombre: String,
    pub descripcion: String,
}
