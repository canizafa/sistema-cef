use crate::app::rol::Rol;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub rol: Rol,
    pub exp: usize,
}
