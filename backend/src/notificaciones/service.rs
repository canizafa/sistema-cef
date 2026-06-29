use crate::{
    app::{
        errors::AppError,
        mailer::{self, Mailer},
    },
    notificaciones::dto::NotificacionRequest,
};

pub async fn notify(request: NotificacionRequest) -> Result<(), AppError> {
    let mailer = mailer::Mailer::new()?;
    Mailer::notify(&mailer, &request.mail, &request.motivo, &request.cuerpo).await?;
    Ok(())
}
