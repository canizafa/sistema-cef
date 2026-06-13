use super::errors::AppError::{self, EnvironmentVariableNotFound};

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_from: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        let port = std::env::var("PORT").map_err(|_| EnvironmentVariableNotFound)?;
        let port: u16 = port.parse().map_err(|_| EnvironmentVariableNotFound)?;
        let database_url =
            std::env::var("DATABASE_URL").map_err(|_| EnvironmentVariableNotFound)?;
        let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| EnvironmentVariableNotFound)?;
        let smtp_host = std::env::var("SMTP_HOST").map_err(|_| EnvironmentVariableNotFound)?;
        let smtp_user = std::env::var("SMTP_USER").map_err(|_| EnvironmentVariableNotFound)?;
        let smtp_pass = std::env::var("SMTP_PASS").map_err(|_| EnvironmentVariableNotFound)?;
        let smtp_from = std::env::var("SMTP_FROM")
            .map_err(|_| EnvironmentVariableNotFound)
            .unwrap_or_else(|_| "noreply@tuapp.com".into());

        Ok(Self {
            port,
            database_url,
            jwt_secret,
            smtp_host,
            smtp_user,
            smtp_pass,
            smtp_from,
        })
    }
}
