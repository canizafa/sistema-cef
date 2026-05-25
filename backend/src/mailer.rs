use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use std::env;

use crate::errors::AppError;

pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
}

impl Mailer {
    pub fn new() -> Result<Self, AppError> {
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_user = env::var("SMTP_USER").expect("SMTP_USER must be set");
        let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
        let smtp_from = env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@tuapp.com".into());

        let creds = Credentials::new(smtp_user, smtp_pass);

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .map_err(|_| AppError::InternalServerError)?
            .credentials(creds)
            .build();

        Ok(Self {
            transport,
            from: smtp_from,
        })
    }

    pub async fn send_new_password(&self, to: &str, new_password: &str) -> Result<(), AppError> {
        let email = Message::builder()
            .from(self.from.parse().map_err(|_| AppError::InternalServerError)?)
            .to(to.parse().map_err(|_| AppError::InternalServerError)?)
            .subject("Tu nueva contraseña")
            .header(ContentType::TEXT_PLAIN)
            .body(format!(
                "Hola,\n\nTu nueva contraseña temporal es:\n\n  {}\n\nTe recomendamos cambiarla al iniciar sesión.\n",
                new_password
            ))
            .map_err(|_| AppError::InternalServerError)?;

        self.transport
            .send(email)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }
}
