use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::Transport;
use lettre_email::EmailBuilder;

use lettre::smtp::ConnectionReuseParameters;
use lettre::SmtpClient;

use lazy_static::lazy_static;

use crate::errors::ApiError;
use crate::models::invitation::Invitation;
use crate::utils::{
    EMAIL_DOMAIN, ORGANISE_NAME, SENDING_EMAIL_ADDRESS, SENDING_EMAIL_PASSWD, SMTP_SERVER_ADDR,
};

use std::fs;
lazy_static! {
    pub static ref EMAIL_TEMPLATE: String = {
        let mut file = std::env::current_dir().unwrap();
        file.push("templates/template.html");
        // println!("{:?}", file);
        fs::read_to_string(file.as_path()).unwrap()
    };
}

pub fn send_email(invit: &Invitation) -> Result<(), ApiError> {
    let email_contents = EMAIL_TEMPLATE
        .clone()
        .replacen("#", ORGANISE_NAME.as_str(), 1)
        .replacen("#", ORGANISE_NAME.as_str(), 1)
        .replacen("#", EMAIL_DOMAIN.as_str(), 1)
        .replacen("#", &invit.id.to_string(), 1)
        .replacen("#", EMAIL_DOMAIN.as_str(), 1)
        .replacen("#", &invit.id.to_string(), 1)
        .replacen("#", EMAIL_DOMAIN.as_str(), 1)
        .replacen("#", &invit.id.to_string(), 1);

    let email = EmailBuilder::new()
        .from(SENDING_EMAIL_ADDRESS.as_str())
        .to(invit.email.as_str())
        .subject("Invitation from Pegasus")
        .alternative(email_contents, "")
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
