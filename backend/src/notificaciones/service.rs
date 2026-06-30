use crate::app::{
    errors::AppError,
    mailer::{self, Mailer},
};
use tracing::instrument;

#[instrument(skip_all, err)]
pub async fn notify(correo: &str, cuerpo: &str, motivo: &str) -> Result<(), AppError> {
    let mailer = mailer::Mailer::new()?;
    Mailer::notify(&mailer, correo, motivo, cuerpo).await?;
    Ok(())
}
