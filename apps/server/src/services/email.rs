use crate::common::settings::RustokSettings;
use async_trait::async_trait;
use loco_rs::{app::AppContext, Error, Result};

use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[derive(Debug, Clone)]
pub struct PasswordResetEmail {
    pub to: String,
    pub reset_url: String,
}

#[async_trait]
pub trait PasswordResetEmailSender: Send + Sync {
    async fn send_password_reset(&self, email: PasswordResetEmail) -> Result<()>;
}

#[derive(Clone)]
pub enum EmailService {
    Disabled,
    Smtp(SmtpEmailSender),
}

impl EmailService {
    pub fn from_ctx(ctx: &AppContext) -> Result<Self> {
        let settings = RustokSettings::from_settings(&ctx.config.settings)
            .map_err(|e| Error::Message(e.to_string()))?;

        if !settings.email.enabled {
            return Ok(Self::Disabled);
        }

        Ok(Self::Smtp(SmtpEmailSender::try_new(&settings)?))
    }

    pub fn password_reset_url(&self, ctx: &AppContext, token: &str) -> Result<String> {
        let settings = RustokSettings::from_settings(&ctx.config.settings)
            .map_err(|e| Error::Message(e.to_string()))?;

        Ok(format!("{}?token={}", settings.email.reset_base_url, token))
    }
}

#[async_trait]
impl PasswordResetEmailSender for EmailService {
    async fn send_password_reset(&self, email: PasswordResetEmail) -> Result<()> {
        match self {
            Self::Disabled => {
                tracing::info!(
                    recipient = %email.to,
                    reset_url = %email.reset_url,
                    "Password reset email provider disabled; reset link emitted to logs"
                );
                Ok(())
            }
            Self::Smtp(sender) => sender.send_password_reset(email).await,
        }
    }
}

#[derive(Clone)]
pub struct SmtpEmailSender {
    from: Mailbox,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpEmailSender {
    fn try_new(settings: &RustokSettings) -> Result<Self> {
        let smtp = &settings.email.smtp;
        let from = settings
            .email
            .from
            .parse::<Mailbox>()
            .map_err(|e| Error::Message(format!("Invalid email.from address: {e}")))?;

        let creds = Credentials::new(smtp.username.clone(), smtp.password.clone());
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp.host)
            .map_err(|e| Error::Message(format!("Invalid SMTP relay config: {e}")))?
            .credentials(creds)
            .port(smtp.port)
            .build();

        Ok(Self { from, transport })
    }
}

#[async_trait]
impl PasswordResetEmailSender for SmtpEmailSender {
    async fn send_password_reset(&self, email: PasswordResetEmail) -> Result<()> {
        let recipient = email
            .to
            .parse::<Mailbox>()
            .map_err(|e| Error::Message(format!("Invalid recipient email address: {e}")))?;

        let message = Message::builder()
            .from(self.from.clone())
            .to(recipient)
            .subject("RusToK password reset")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                "<p>You requested a password reset.</p><p><a href=\"{}\">Reset password</a></p>",
                email.reset_url
            ))
            .map_err(|e| Error::Message(format!("Failed to build reset email: {e}")))?;

        self.transport
            .send(message)
            .await
            .map_err(|e| Error::Message(format!("Failed to send reset email: {e}")))?;

        Ok(())
    }
}
