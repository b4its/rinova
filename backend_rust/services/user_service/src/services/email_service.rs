use lettre::{
    message::Mailbox,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::env;

pub async fn send_verification_email(email: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        .subject("Verify Your Email - Rinova Website Builder")
        .body(String::from("Please click the verification link to complete your registration."))?;

    mailer.send(email_message).await?;

    Ok(())
}

pub async fn send_welcome_email(email: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
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
