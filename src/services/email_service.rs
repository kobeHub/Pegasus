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

/*
    // TODO: hard-code to be fixed
    let email_body = format!(
        "Please click on the link below to complete registration. <br/>
         <a href=\"http://localhost:8088/register.html?id={}&email={}\">
         http://localhost:3030/register</a> <br>
         your Invitation expires on <strong>{}</strong>",
        invit.id,
        invit.email,
        invit.expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );
 */

/*
#[derive(Debug, Serialize)]
pub struct Contact {
    email: String,
    name: Option<String>,
}

impl Contact {
    pub fn new<T: Into<String>>(email: T, name: T) -> Self {
        Contact {
            email: email.into(),
            Some(name.into()),
        }
    }
}

impl<T: Into<String>> From<T> for Contact {
    fn from(email: T) -> Self {
        Contact {
            email: email.into(),
            None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Email {
    sender: Contact,
    #[serde(rename = "to")]
    recipient: Contact,
    subject: String,
    #[serde(rename = "htmlContent")]
    html: Option<String>,
}

impl Email {
    pub fn new(sender: Contact, recipient: Contact) -> Self {
        Email {
            sender,
            recipient,
            subject: "".to_string(),
            html: None,
        }
    }

    pub fn set_subject<T: Into<String>>(mut self, subject: T) -> Self {
        self.subject = subject.into();
        self
    }

    pub fn set_html<T: Into<String>>(mut self, html: T) -> Self {
        self.html = Some(html.into());
        self
    }
}
*/
