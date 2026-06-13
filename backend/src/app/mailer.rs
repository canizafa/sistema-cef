use crate::app::AppError;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::header::ContentType,
    transport::smtp::{SMTP_PORT, authentication::Credentials},
};
use std::env;

pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
}

impl Mailer {
    pub fn new() -> Result<Self, AppError> {
        let smtp_host = env::var("SMTP_HOST").map_err(|_| AppError::EnvironmentVariableNotFound)?;
        let smtp_user = env::var("SMTP_USER").map_err(|_| AppError::EnvironmentVariableNotFound)?;
        let smtp_pass = env::var("SMTP_PASS").map_err(|_| AppError::EnvironmentVariableNotFound)?;
        let smtp_from = env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@tuapp.com".into());

        let creds = Credentials::new(smtp_user, smtp_pass);

        let smtp_port: u16 = env::var("SMTP_PORT")
            .map(|p| p.parse().unwrap_or(SMTP_PORT))
            .unwrap_or(SMTP_PORT);

        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_host)
            .map_err(|_| AppError::Internal)?
            .credentials(creds)
            .port(smtp_port)
            .build();

        Ok(Self {
            transport,
            from: smtp_from,
        })
    }

    pub async fn send_new_password(&self, to: &str, new_password: &str) -> Result<(), AppError> {
        let email = Message::builder()
            .from(self.from.parse().map_err(|_| AppError::Internal)?)
            .to(to.parse().map_err(|_| AppError::Internal)?)
            .subject("Tu nueva contraseña")
            .header(ContentType::TEXT_PLAIN)
            .body(format!(
                "Hola,\n\nTu nueva contraseña temporal es:\n\n  {}\n\nTe recomendamos cambiarla al iniciar sesión.\n",
                new_password
            ))
            .map_err(|_| AppError::Internal)?;

        self.transport
            .send(email)
            .await
            .map_err(|_| AppError::Internal)?;

        Ok(())
    }
}
