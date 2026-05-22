use serde::{Deserialize, Serialize};

use crate::domain::rol::Rol;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub rol: Rol,
    pub exp: usize,
}
