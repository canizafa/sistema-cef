use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Estado {
    Alta,
    Baja,
}

#[derive(Deserialize, Serialize)]
pub struct CrearProfesor {
    pub dni: i32,
    pub nombre: String,
    pub pellido: String,
    pub genero: String,
    pub estado: Estado,
}

#[derive(Deserialize, Serialize)]
pub struct Profesor {
    pub id: i32,
    pub dni: i32,
    pub nombre: String,
    pub pellido: String,
    pub genero: String,
    pub estado: Estado,
}
