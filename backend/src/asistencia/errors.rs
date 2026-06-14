use thiserror::Error;

#[derive(Debug, Error)]
pub enum AsistenciaDomainError {
    #[error("Asistencia invalida")]
    InvalidAsistencia,
}
