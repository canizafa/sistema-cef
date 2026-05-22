use crate::domain::rol::Rol;

pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub struct AuthResponse {
    pub access_token: String,
    pub rol: Rol,
}
