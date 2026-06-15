use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmpleadoDomainError {
    #[error("DNI inválido")]
    InvalidDNI,
    #[error("Email inválido")]
    InvalidEmail,
    #[error("Contraseña débil")]
    WeakPassword,
}
