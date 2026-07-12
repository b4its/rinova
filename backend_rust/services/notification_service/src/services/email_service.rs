//! Email notification service

use anyhow::{Context, Result};
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};
use uuid::Uuid;

/// Email configuration
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

impl EmailConfig {
    /// Load from environment
    pub fn from_env() -> Result<Self> {
        Ok(EmailConfig {
            smtp_host: std::env::var("SMTP_HOST")
                .unwrap_or_else(|_| "smtp.example.com".to_string()),
            smtp_port: std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .context("Invalid SMTP_PORT")?,
            smtp_username: std::env::var("SMTP_USERNAME")
                .unwrap_or_default(),
            smtp_password: std::env::var("SMTP_PASSWORD")
                .unwrap_or_default(),
            from_email: std::env::var("FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@rinova.app".to_string()),
            from_name: std::env::var("FROM_NAME")
                .unwrap_or_else(|_| "Rinova".to_string()),
        })
    }
}

/// Email service for sending notifications
pub struct EmailService {
    config: EmailConfig,
    mailer: Option<SmtpTransport>,
}

impl EmailService {
    /// Create a new email service
    pub fn new(config: EmailConfig) -> Self {
        let mailer = if !config.smtp_host.is_empty() && !config.smtp_username.is_empty() {
            let credentials = Credentials::new(
                config.smtp_username.clone(),
                config.smtp_password.clone(),
            );

            Some(
                SmtpTransport::relay(&config.smtp_host)
                    .unwrap()
                    .credentials(credentials)
                    .port(config.smtp_port)
                    .build(),
            )
        } else {
            None
        };

        EmailService { config, mailer }
    }

    /// Send a workspace invitation email
    pub async fn send_workspace_invitation(
        &self,
        to_email: &str,
        workspace_id: Uuid,
        role: &str,
    ) -> Result<()> {
        let subject = "You've been invited to join a workspace on Rinova";
        let body = format!(
            r#"
You have been invited to join a workspace as {}.

Click the link below to accept the invitation:
https://rinova.app/workspaces/{}/join

If you did not expect this invitation, you can safely ignore this email.

Best regards,
The Rinova Team
"#,
            role, workspace_id
        );

        self.send_email(to_email, subject, &body).await
    }

    /// Send a publish confirmation email
    pub async fn send_publish_confirmation(
        &self,
        to_email: &str,
        project_name: &str,
        url: &str,
    ) -> Result<()> {
        let subject = format!("{} is now live!", project_name);
        let body = format!(
            r#"
Great news! Your website {} has been successfully published.

Your website is now live at: {}

Thank you for using Rinova!

Best regards,
The Rinova Team
"#,
            project_name, url
        );

        self.send_email(to_email, subject, &body).await
    }

    /// Send a generic email
    pub async fn send_email(&self, to_email: &str, subject: &str, body: &str) -> Result<()> {
        tracing::info!("Sending email to: {}", to_email);

        if let Some(ref mailer) = self.mailer {
            let from: Mailbox = format!("{} <{}>", self.config.from_name, self.config.from_email)
                .parse()
                .context("Invalid from email")?;

            let to: Mailbox = to_email
                .parse()
                .with_context(|| format!("Invalid to email: {}", to_email))?;

            let email = Message::builder()
                .from(from)
                .to(to)
                .subject(subject)
                .body(body.to_string())
                .context("Failed to build email")?;

            mailer
                .send(&email)
                .context("Failed to send email")?;

            tracing::info!("Email sent successfully to: {}", to_email);
        } else {
            tracing::warn!("Email not configured, skipping email to: {}", to_email);
        }

        Ok(())
    }
}

impl Default for EmailService {
    fn default() -> Self {
        EmailService {
            config: EmailConfig::from_env().unwrap_or_else(|_| EmailConfig {
                smtp_host: String::new(),
                smtp_port: 587,
                smtp_username: String::new(),
                smtp_password: String::new(),
                from_email: "noreply@rinova.app".to_string(),
                from_name: "Rinova".to_string(),
            }),
            mailer: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_service_creation() {
        let service = EmailService::default();
        assert!(service.mailer.is_none()); // No SMTP configured in tests
    }
}
