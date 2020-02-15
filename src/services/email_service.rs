use lettre_email::EmailBuilder;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::Transport;

use lettre::SmtpClient;
use lettre::smtp::ConnectionReuseParameters;

use crate::errors::ApiError;
use crate::models::invitation::Invitation;
use crate::utils::{SENDING_EMAIL_ADDRESS,
                   SENDING_EMAIL_PASSWD,
                   SMTP_SERVER_ADDR, ORGANISE_NAME};


pub fn send_email(invit: &Invitation) -> Result<(), ApiError> {
    let email = EmailBuilder::new()
        .from(SENDING_EMAIL_ADDRESS.as_str())
        .to(invit.email.as_str())
        .subject("test")
        .body("test")
        .build()?;

    let mut transport = SmtpClient::new_simple(&SMTP_SERVER_ADDR)?
        .smtp_utf8(true)
        .hello_name(ClientId::Domain(ORGANISE_NAME.clone()))
        .credentials(Credentials::new(
            SENDING_EMAIL_ADDRESS.clone(),
            SENDING_EMAIL_PASSWD.clone(),
        ))
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

    transport.send(email.into())?;
    Ok(())
}
