use serde::{Deserialize, Serialize};

use crate::domain::rol::Rol;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub rol: Rol,
    pub exp: usize,
}
