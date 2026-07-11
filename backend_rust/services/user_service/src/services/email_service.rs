use lettre::{
    message::Mailbox,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::env;
use tokio::time::{timeout, Duration};
use shared::errors::ServiceError;
use shared::ServiceResult;

const EMAIL_SEND_TIMEOUT_SECS: u64 = 30;

pub async fn send_verification_email(email: &str, token: &str) -> ServiceResult<()> {
    // Wrap email sending in a timeout to ensure it completes within 30 seconds
    let email_owned = email.to_string();
    let token_owned = token.to_string();
    
    let result = timeout(
        Duration::from_secs(EMAIL_SEND_TIMEOUT_SECS),
        send_verification_email_internal(&email_owned, &token_owned)
    ).await;

    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(ServiceError::EmailError(e.to_string())),
        Err(_) => Err(ServiceError::EmailError("Email sending timed out after 30 seconds".to_string())),
    }
}

async fn send_verification_email_internal(email: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let smtp_port: u16 = env::var("SMTP_PORT")
        .unwrap_or_else(|_| "587".to_string())
        .parse()
        .unwrap_or(587);
    let smtp_user = env::var("SMTP_USER")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;

    let credentials = Credentials::new(smtp_user.clone(), smtp_password);

    let mailer: AsyncSmtpTransport<Tokio1Executor> = 
        AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)?
            .credentials(credentials)
            .port(smtp_port)
            .build();

    let from_email = env::var("SMTP_FROM_EMAIL")
        .unwrap_or_else(|_| smtp_user.clone());

    // Get frontend URL from env or use default
    let frontend_url = env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    // Construct verification link
    let verification_link = format!("{}/auth/verify-email?token={}", frontend_url, token);

    let email_message = Message::builder()
        .from(Mailbox::new(
            Some("Rinova".to_string()),
            from_email.parse()?,
        ))
        .to(email.parse()?)
        .subject("Verify Your Email - Rinova Website Builder")
        .body(format!(
            r#"
Welcome to Rinova Website Builder!

Please verify your email address by clicking the link below:

{}

This verification link will expire in 24 hours.

If you did not create an account with Rinova, please ignore this email.

Best regards,
The Rinova Team
"#,
            verification_link
        ))?;

    mailer.send(email_message).await?;

    Ok(())
}

pub async fn send_welcome_email(email: &str, name: &str) -> ServiceResult<()> {
    // Wrap email sending in a timeout
    let result = timeout(
        Duration::from_secs(EMAIL_SEND_TIMEOUT_SECS),
        send_welcome_email_internal(email, name)
    ).await;

    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(ServiceError::EmailError(e.to_string())),
        Err(_) => Err(ServiceError::EmailError("Email sending timed out after 30 seconds".to_string())),
    }
}

async fn send_welcome_email_internal(email: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let smtp_port: u16 = env::var("SMTP_PORT")
        .unwrap_or_else(|_| "587".to_string())
        .parse()
        .unwrap_or(587);
    let smtp_user = env::var("SMTP_USER")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;

    let credentials = Credentials::new(smtp_user.clone(), smtp_password);

    let mailer: AsyncSmtpTransport<Tokio1Executor> = 
        AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)?
            .credentials(credentials)
            .port(smtp_port)
            .build();

    let from_email = env::var("SMTP_FROM_EMAIL")
        .unwrap_or_else(|_| smtp_user.clone());

    let email_message = Message::builder()
        .from(Mailbox::new(
            Some("Rinova".to_string()),
            from_email.parse()?,
        ))
        .to(email.parse()?)
        .subject("Welcome to Rinova Website Builder!")
        .body(format!("Welcome, {}! Start building your website today.", name))?;

    mailer.send(email_message).await?;

    Ok(())
}
