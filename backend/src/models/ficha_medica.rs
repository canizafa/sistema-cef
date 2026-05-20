use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FichaMedica {
    pub enfermedades: bool,
    pub operaciones_quirurgicas: bool,
    pub detalles: String,
}
