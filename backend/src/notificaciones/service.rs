use crate::{
    app::{
        errors::AppError,
        mailer::{self, Mailer},
    },
    notificaciones::dto::NotificacionRequest,
};
use tracing::instrument;

#[instrument(skip_all, err)]
pub async fn notify(request: NotificacionRequest) -> Result<(), AppError> {
    let mailer = mailer::Mailer::new()?;
    Mailer::notify(&mailer, &request.email, &request.motivo, &request.cuerpo).await?;
    Ok(())
}
